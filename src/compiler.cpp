// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/import_locator.hpp"
#include <atomic>
#include <cstdint>
#include <exception>
#include <filesystem>
#include <iterator>
#include <optional>
#include <spdlog/fmt/bundled/format.h>
#include <thread>
#include <vector>
#define BS_THREAD_POOL_NATIVE_EXTENSIONS
#include "slingshot/compiler.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/lang_lifter.hpp"
#include "slingshot/slingshot.hpp"
#include <BS_thread_pool.hpp>
#include <ankerl/unordered_dense.h>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <slang/analysis/AnalysisManager.h>
#include <slang/ast/Compilation.h>
#include <slang/ast/symbols/CompilationUnitSymbols.h>
#include <slang/diagnostics/DiagnosticEngine.h>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/driver/Driver.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>

// Parts of this are based on slang-server ServerDiagClient.cpp, which is available under the MIT licence:
//
// Copyright (c) 2024-2025 Hudson River Trading LLC <opensource@hudson-trading.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

using namespace slingshot;
using namespace slang::syntax;
using namespace slang::ast;
using namespace slang::analysis;

void LSPDiagnosticClient::report(const ReportedDiagnostic &diagnostic) {
    SPDLOG_TRACE("Received a diagnostic");

    {
        auto lock = g_compilerManager.acquireLock();

        if (!g_compilerManager.bufferIdsInverse.contains(diagnostic.location.buffer())) {
            SPDLOG_ERROR("Diagnostic in buffer ID {} could not be found in bufferIdsInverse",
                diagnostic.location.buffer().getId());
            return;
        }

        auto path = g_compilerManager.bufferIdsInverse.at(diagnostic.location.buffer());
        if (path != targetPath) {
            SPDLOG_TRACE("Diagnostic in buffer {} path {} did not match target path {}",
                diagnostic.location.buffer().getId(), path.string(), targetPath.string());
            return;
        }
    }

    lsp::Diagnostic lspDiag;
    switch (diagnostic.severity) {
        case slang::DiagnosticSeverity::Note:
            lspDiag.severity = lsp::DiagnosticSeverity::Information;
            break;

        case slang::DiagnosticSeverity::Warning:
            lspDiag.severity = lsp::DiagnosticSeverity::Warning;
            break;

        case slang::DiagnosticSeverity::Error:
        case slang::DiagnosticSeverity::Fatal:
            lspDiag.severity = lsp::DiagnosticSeverity::Error;
            break;

        default:
            SPDLOG_ERROR("Unhandled Slang diagnostic severity");
            break;
    }

    lspDiag.message = diagnostic.formattedMessage;
    lspDiag.source = "Slang (via Slingshot)";

    // Code similar to TextDiagnosticClient::report
    SmallVector<SourceRange> mappedRanges;
    engine->mapSourceRanges(diagnostic.location, diagnostic.ranges, mappedRanges);

    auto mainLoc = getLocation(diagnostic.location, mappedRanges, diagnostic.formattedMessage, sourceMgr);
    if (!mainLoc) {
        return;
    }

    std::vector<lsp::DiagnosticRelatedInformation> related;
    for (auto it = diagnostic.expansionLocs.rbegin(); it != diagnostic.expansionLocs.rend(); it++) {
        SourceLocation loc = *it;
        std::string name(sourceManager->getMacroName(loc));
        if (name.empty()) {
            name = "expanded from here";
        } else {
            name = fmt::format("expanded from macro '{}'", name);
        }

        SmallVector<SourceRange> macroRanges;
        engine->mapSourceRanges(loc, diagnostic.ranges, macroRanges);

        auto relatedLoc = getLocation(sourceManager->getFullyOriginalLoc(loc), macroRanges, name, sourceMgr);
        if (relatedLoc) {
            related.emplace_back(lsp::DiagnosticRelatedInformation {
                .location = *relatedLoc, .message = std::string { diagnostic.formattedMessage } });
        }
    }
    // end of text diag related code
    lspDiag.range = mainLoc->range;

    lspDiags.push_back(lspDiag);
}

void CompilationManager::submitCompilationJob(
    const std::string &document, const std::filesystem::path &path, bool isIndex) {

    SourceBuffer buf;

    {
        auto lock = acquireLock();

        // FIXME this may leak memory
        buf = sourceMgr->assignText(document);

        bufferIds[path] = buf.id;
        bufferIdsInverse[buf.id] = path;
        bufMap[path] = buf;
    }

    auto now = timeNowNs();

    pool.detach_task([buf, path, this, document, now, isIndex] {
        try {
            SPDLOG_DEBUG("Submitting document {} for compilation", path.string());

            BS::this_thread::set_os_thread_name("Compiler");

            {
                // is the data out of date?
                auto entry = g_indexManager.retrieve(path);
                // if the last updated time is AFTER our compilation job was submitted, we know our data is
                // old
                if (entry.has_value()) {
                    auto entryTime = (*entry)->lastUpdated;
                    if (entryTime > now) {
                        SPDLOG_WARN(
                            "Dropping old compilation job! Our time: {}, entry time: {}", now, entryTime);
                        return;
                    }
                    SPDLOG_TRACE("Compilation job is valid, we can proceed");
                }
            }

            SPDLOG_TRACE("==== COMPILING {} contents: =====\n{}", path.string(), document);

            if (isIndex && g_indexManager.isInitialIndexInProgress) {
                sendLspProgressMsg("Indexing " + path.string());
            }

            // setup the diagnostics engine
            DiagnosticEngine diagEngine { *sourceMgr };
            // get more diagnostics
            diagEngine.setIgnoreAllNotes(false);
            diagEngine.setIgnoreAllWarnings(false);

            // this is our custom listener for diagnostics that we'll filter and report to the LSP
            LSPDiagnosticClient::Ptr diagClient = std::make_shared<LSPDiagnosticClient>(path);
            diagClient->setSourceManager(sourceMgr);
            diagEngine.addClient(diagClient);

            // do initial CST parse, no dependencies, nothing
            auto tree = doCstParse(path, buf, diagEngine);

            // if we're doing the initial index, make SURE that we update the import table before we perform
            // AST compilation. AST compilation will complain if we have no import table.
            if (isIndex) {
                reIndexDocument(path, tree, true); // in index, so send notif
            }

            // do AST parse
            auto compilation = doAstParse(path, buf, diagEngine, tree);
            if (compilation == nullptr) {
                // the compilation job failed, likely because we couldn't find the symbols we were looking for
                // in this document, and that's probably because they're already being compiled in another
                // thread
                // let's wait for a second to ensure that work gets done, then re-submit and try again.
                SPDLOG_WARN("Failed to compile document: {}. Resubmitting job in 1s.", path.string());
                std::this_thread::sleep_for(1s);
                submitCompilationJob(document, path, isIndex);
                return;
            }

            // also perform analysis
            doAnalysis(buf, diagEngine, compilation);

            // lift to our own internal higher level representation for completion
            doLifting(path, tree);

            // enqueue outgoing diagnostics
            outgoingDiagnostics.enqueue({ .timestamp = timeNowNs(), .path = path, .lspDiags = diagClient });

            if (isIndex) {
                indexingJobsInProgress--;
            }

            // also check here if the indexing is done, in case it failed earlier
            maybeFinaliseIndexingProgress();
        } catch (const std::exception &e) {
            SPDLOG_ERROR("Caught exception in compilation job: {}", e.what());
        }
    });
}

std::shared_ptr<slang::syntax::SyntaxTree> CompilationManager::doCstParse(
    const std::filesystem::path &path, const SourceBuffer &buf, DiagnosticEngine &diagEngine) {
    auto tree = SyntaxTree::fromBuffer(buf, *sourceMgr);
    SPDLOG_TRACE("Parsed document {}, got {} CST diagnostics", path.string(), tree->diagnostics().size());
    // this is essential so that later, we will have the parse tree associated with this current
    // document
    g_indexManager.associateParse(path, tree);

    // first, issue syntax diagnostics we got in parsing
    for (const auto &diag : tree->diagnostics()) {
        diagEngine.issue(diag);
    }

    return tree;
}

std::shared_ptr<ast::Compilation> CompilationManager::doAstParse(const std::filesystem::path &path,
    const SourceBuffer &buf, DiagnosticEngine &diagEngine,
    const std::shared_ptr<slang::syntax::SyntaxTree> &tree) {
    // try and get the default driver options, which seem to be a necessity to get diagnostics, which
    // is our only goal here atm
    driver::Driver slangDriver;
    slangDriver.addStandardArgs();
    slangDriver.options.errorLimit = 100;
    slangDriver.options.maxGenerateSteps = 32;
    auto options = slangDriver.createOptionBag();

    // create a compilation, so we can get further diagnostics; this will yield for us the AST,
    // whereas before we had the CST

    auto compilation = std::make_shared<Compilation>(options);
    // only initially add the document itself as a syntax tree, we'll discover the other documents later
    compilation->addSyntaxTree(tree);

    auto lock = acquireLock();
    if (!requiredDocuments.contains(path)) {
        SPDLOG_WARN("Required documents for path {} are unknown!", path.string());
    } else {
        // determine if the document graph needs rebuilding, by checking the import table
        // this will occur e.g. if the user adds another module, changes imports, etc.
        // computing the import table is quick enough we can do it here.
        auto imports = ImportLocator::locateRequiredProvidedImports(tree, path);
        if (importHashes[path] != imports.hash() && !g_indexManager.isInitialIndexInProgress) {
            SPDLOG_WARN("Import table changed outside of indexing, document graph must be rebuilt!");
            // this unlock and relock shenanigan is necessary to avoid deadlocks, at least according to
            // ThreadSanitizer
            lock.unlock();
            // assume indexing done, don't send LSP notification
            reIndexDocument(path, tree, false);
            lock.lock();
        }

        auto docs = requiredDocuments.at(path);
        // NOTE extremely important we unlock here to prevent deadlocks (see sling issue #72)
        lock.unlock();
        for (const auto &doc : docs) {
            auto result = g_indexManager.retrieve(doc);
            // we need an index lock before we can read, see:
            // https://github.com/mlyoung101/slingshot/issues/72#issuecomment-3858321765
            auto lock = g_indexManager.acquireLock();
            if (result.has_value() && result != std::nullopt && (*result)->tree != nullptr) {
                SPDLOG_TRACE("{} ---(requires)---> {}", path.string(), doc.string());
                compilation->addSyntaxTree((*result)->tree);
            } else {
                SPDLOG_WARN("Required document {} present in CompilerManager, but could not retrieve from "
                            "IndexManager",
                    doc.string());
            }
        }
        SPDLOG_TRACE(
            "Compilation for {} has {} syntax trees", path.string(), compilation->getSyntaxTrees().size());
    }

    // finalise it, apparently we have to call getRoot() to do this
    SPDLOG_TRACE("Finalise AST compilation");
    compilation->getRoot();
    for (const auto &diag : compilation->getAllDiagnostics()) {
        SPDLOG_TRACE("Got an AST diagnostic {} when parsing {}", slang::toString(diag.code), path.string());
        // ensure the diagnostic relates to the file we're compiling
        if (diag.location.buffer() == buf.id) {
            SPDLOG_TRACE("Issued a diagnostic in the AST");
            diagEngine.issue(diag);
        }
    }

    compilation->freeze();

    return compilation;
}

void CompilationManager::doAnalysis(
    const SourceBuffer &buf, DiagnosticEngine &diagEngine, std::shared_ptr<ast::Compilation> &compilation) {
    SPDLOG_TRACE("Perform analysis");
    AnalysisManager analysisMgr;
    analysisMgr.analyze(*compilation);
    for (const auto &diag : analysisMgr.getDiagnostics()) {
        SPDLOG_TRACE("Got an analysis diagnostic");
        // ensure the diagnostic relates to the file we're compiling
        if (diag.location.buffer() == buf.id) {
            SPDLOG_DEBUG("Issued a diagnostic in analysis");
            diagEngine.issue(diag);
        }
    }
}

// NOLINTNEXTLINE(readability-convert-member-functions-to-static)
void CompilationManager::doLifting(
    const std::filesystem::path &path, std::shared_ptr<slang::syntax::SyntaxTree> &tree) {
    SPDLOG_TRACE("Lifting language");
    LangLifterVisitor langLifter;
    langLifter.visit(tree->root());
    langLifter.doc.maybeFlushModule();

    SPDLOG_TRACE("Associate slingshot::lang document");
    g_indexManager.associateLangDoc(path, langLifter.doc);
}

void CompilationManager::issueDiagnostics(
    const std::filesystem::path &path, const LSPDiagnosticClient::Ptr &diagClient) {
    // we only do this if the text document is open, to avoid extraneous errors
    auto lock = acquireLock();
    if (openFiles.contains(path)) {
        SPDLOG_DEBUG("Issue {} diagnostics to client for buffer {}", diagClient->getLspDiagnostics().size(),
            path.string());

        // NOTE: this is absolutely fucking ridiculous, but is apparently required to stop Neovim from
        // ignoring our push diagnostic messages?! are we fucking insane?! why does this work?!
        // see: https://github.com/mlyoung101/slingshot/issues/76
        for (int i = 0; i < 3; i++) {
            lsp::notifications::TextDocument_PublishDiagnostics::Params lspDiagMsg;
            lspDiagMsg.diagnostics = diagClient->getLspDiagnostics();
            lspDiagMsg.uri = lsp::Uri::parse("file://" + path.string());

            g_msgHandler->sendNotification<lsp::notifications::TextDocument_PublishDiagnostics>(
                std::move(lspDiagMsg));
        }
    }
}

void CompilationManager::maybeFinaliseIndexingProgress() {
    // we want to check if indexing is done, but if we're the first task submitted, we'll be
    // like "oh, there's no jobs here! we're done!". so, we introduce another atomic variable that
    // keeps track of _if_ we're still queueing indexing jobs, which is controlled from indexing.cpp
    if (g_indexManager.isInitialIndexInProgress && !g_indexManager.isStillQueueingIndexJobs
        && indexingJobsInProgress <= 0) {
        // then we can submit a work done progress end, we've finished everything
        SPDLOG_INFO("Indexing believed to be done!");

        // since we've just finished, submit a bulk compilation job
        pool.detach_task([this] { performBulkCompilation(true); });
    }
}

// REQUIRES ITS OWN LOCK
void CompilationManager::locateAllRequiredDocuments(bool shouldSendLspNotification) {
    SPDLOG_INFO("Locating all required documents");

    // finalise the graph
    g_indexManager.documentGraph->finaliseOutstandingSymbols();

    if (g_indexManager.documentGraph->doesHaveCycles()) {
        SPDLOG_ERROR("Dependency graph is malformed and has cycles! Cannot compute dependents!");
        SPDLOG_ERROR(
            "This WILL break the index, you need to fix this by removing self-referential dependencies.");
        SPDLOG_ERROR("This may assist you:");
        g_indexManager.documentGraph->debugLocateCycles();
        // TODO warn user in GUI
        return;
    }

    for (const auto &doc : g_indexManager.documentGraph->getAllKnownDocuments()) {
        requiredDocuments[doc] = g_indexManager.documentGraph->locateRequiredDependents(doc);
    }
}

void CompilationManager::performBulkCompilation(bool shouldSendLspNotification) {
    SPDLOG_INFO("Performing bulk compilation");

    auto indexLock = g_indexManager.acquireLock();
    auto compilerLock = acquireLock();

    if (shouldSendLspNotification) {
        sendLspProgressMsg("Computing dependents from document graph");
    }

    // TODO do this at the end or only once? it takes a while now
    locateAllRequiredDocuments(shouldSendLspNotification);

    // we also need to recompile all the open files now, to clear out all the warnings
    SPDLOG_DEBUG("Recompiling documents now that indexing is done");
    for (const auto &doc : openFiles) {
        // do NOT pull from disk, pull from the index!
        // see: https://github.com/mlyoung101/slingshot/issues/76
        reCompileDocument(doc);
    }

    // FIXME we should only send this once
    if (shouldSendLspNotification) {
        lsp::notifications::Progress::Params endMsg;
        endMsg.token = "SlingshotIndexProgress";
        endMsg.value = lsp::toJson(lsp::WorkDoneProgressEnd());
        g_msgHandler->sendNotification<lsp::notifications::Progress>(std::move(endMsg));
        g_indexManager.isInitialIndexInProgress = false;
    }
}

void CompilationManager::reIndexDocument(const std::filesystem::path &path,
    const std::shared_ptr<slang::syntax::SyntaxTree> &tree, bool shouldSendLspNotification) {
    SPDLOG_TRACE("Reindexing document: {}, contents:{}\n", path.string(), tree->root().toString());

    // figure out what symbols this document provides and requires
    auto imports = ImportLocator::locateRequiredProvidedImports(tree, path);
    {
        auto indexLock = g_indexManager.acquireLock();
        auto compilerLock = acquireLock();
        for (const auto &provided : imports.providedSymbols) {
            g_indexManager.documentGraph->registerProvidedSymbol(path, provided);
        }
        for (const auto &required : imports.requiredSymbols) {
            g_indexManager.documentGraph->registerRequiredSymbol(path, required);
        }
        for (const auto &required : imports.maybeRequiredSymbols) {
            g_indexManager.documentGraph->registerMaybeRequiredSymbol(path, required);
        }
        importHashes[path] = imports.hash();

        // always attempt to locate outstanding symbols and process the required documents list
        locateAllRequiredDocuments(shouldSendLspNotification);
    }
}

// FIXME if we were smart (which we're not), this should mostly be inlined into submitCompilationJob; it
// stinks that we're duplicating it here
void CompilationManager::reCompileDocument(const std::filesystem::path &path) {
    auto now = timeNowNs();

    pool.detach_task([path, this, now] {
        try {
            SPDLOG_DEBUG("Recompiling document after re-index: {}", path.string());

            BS::this_thread::set_os_thread_name("Compiler");

            {
                // is the data out of date?
                auto entry = g_indexManager.retrieve(path);
                // if the last updated time is AFTER our compilation job was submitted, we know our data is
                // old
                if (entry.has_value()) {
                    auto entryTime = (*entry)->lastUpdated;
                    if (entryTime > now) {
                        SPDLOG_WARN(
                            "Dropping old compilation job! Our time: {}, entry time: {}", now, entryTime);
                        return;
                    }
                    SPDLOG_TRACE("Compilation job is valid, we can proceed");
                }
            }

            SourceBuffer buf;
            {
                auto lock = acquireLock();
                if (!bufMap.contains(path)) {
                    SPDLOG_ERROR("bufMap does not contain path {}, cannot reindex!", path.string());
                    return;
                }
                buf = bufMap.at(path);
            }

            SPDLOG_TRACE("==== RE-COMPILING {}", path.string());

            // setup the diagnostics engine
            DiagnosticEngine diagEngine { *sourceMgr };
            // get more diagnostics
            diagEngine.setIgnoreAllNotes(false);
            diagEngine.setIgnoreAllWarnings(false);

            // this is our custom listener for diagnostics that we'll filter and report to the LSP
            LSPDiagnosticClient::Ptr diagClient = std::make_shared<LSPDiagnosticClient>(path);
            diagClient->setSourceManager(sourceMgr);
            diagEngine.addClient(diagClient);

            // do initial CST parse
            auto tree = doCstParse(path, buf, diagEngine);

            // do AST parse
            auto compilation = doAstParse(path, buf, diagEngine, tree);
            if (compilation == nullptr) {
                SPDLOG_WARN("Failed to re-compile document: {}!", path.string());
                return;
            }

            // also perform analysis
            doAnalysis(buf, diagEngine, compilation);

            // lift to our own internal higher level representation for completion
            doLifting(path, tree);

            // enqueue diagnostics
            outgoingDiagnostics.enqueue({ .timestamp = timeNowNs(), .path = path, .lspDiags = diagClient });
        } catch (const std::exception &e) {
            SPDLOG_ERROR("Caught exception in re-compilation job: {}", e.what());
        }
    });
}

void CompilationManager::startOutgoingDiagnostics() {
    SPDLOG_INFO("Booting outgoing diagnostics thread");

    auto thread = std::thread(&CompilationManager::outgoingDiagnosticsThread, this);
    pthread_setname_np(thread.native_handle(), "DiagOut");
    thread.detach();
}

void CompilationManager::outgoingDiagnosticsThread() {
    SPDLOG_DEBUG("Enter outgoingDiagnosticsThread");

    ankerl::unordered_dense::map<std::filesystem::path, uint64_t> lastTimes;

    while (running) {
        TimestampedDiagnostics diag;
        outgoingDiagnostics.wait_dequeue(diag);

        if (lastTimes.contains(diag.path) && lastTimes.at(diag.path) > diag.timestamp) {
            SPDLOG_WARN("Discarding old diagnostic for path: {}", diag.path.string());
            continue;
        }

        issueDiagnostics(diag.path, diag.lspDiags);
        lastTimes[diag.path] = diag.timestamp;

        // wait for 100 ms (rate limit!)
        std::this_thread::sleep_for(100ms);
    }

    SPDLOG_DEBUG("outgoingDiagnosticsThread() terminating");
}

std::string CompilationManager::debugGetDepsForFile(const std::string &path) {
    auto lock = acquireLock();
    if (!requiredDocuments.contains(path)) {
        return fmt::format("Not found in {} deps", requiredDocuments.size());
    }

    std::string out = fmt::format("Required documents for path {}\n", path);
    size_t i = 0;
    for (const auto &req : requiredDocuments[path]) {
        out += fmt::format("  {}. {}\n", i++, req.string());
    }
    return out;
}
