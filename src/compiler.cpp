// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/import_locator.hpp"
#include <algorithm>
#include <exception>
#include <filesystem>
#include <iterator>
#include <optional>
#include <random>
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
        auto lock = g_compilerManager.acquireReadLock();

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
    const std::string &document, const std::filesystem::path &path) {

    SourceBuffer buf;

    {
        auto lock = acquireWriteLock();

        // FIXME this may leak memory
        buf = sourceMgr->assignText(document);

        bufferIds[path] = buf.id;
        bufferIdsInverse[buf.id] = path;
    }

    pool.detach_task([buf, path, this, document] {
        try {
            SPDLOG_DEBUG("Submitting document {} for compilation", path.string());

            BS::this_thread::set_os_thread_name("Compiler");

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
                // the compilation job failed, likely because we couldn't find the symbols we were looking for
                // in this document, and that's probably because they're already being compiled in another
                // thread
                // let's wait for a second to ensure that work gets done, then re-submit and try again.
                SPDLOG_WARN("Failed to compile document: {}. Resubmitting job in 1s.", path.string());
                std::this_thread::sleep_for(1s);
                submitCompilationJob(document, path);
                return;
            }

            // also perform analysis
            doAnalysis(buf, diagEngine, compilation);

            // lift to our own internal higher level representation for completion
            doLifting(path, tree);

            // publish diagnostics to the client
            issueDiagnostics(path, diagClient);

            // also check here if the indexing is done, in case it failed earlier
            maybeFinaliseIndexingProgress();
        } catch (const std::exception &e) {
            SPDLOG_ERROR("Caught exception in compilation job: {}", e.what());
        }
    });
}

void CompilationManager::indexDocument(const std::string &document, const std::filesystem::path &path) {
    SourceBuffer buf;
    indexingJobsInProgress++;

    {
        auto lock = acquireWriteLock();

        // FIXME this may leak memory
        buf = sourceMgr->assignText(document);

        bufferIds[path] = buf.id;
        bufferIdsInverse[buf.id] = path;
    }

    pool.detach_task([buf, path, this, document] {
        try {
            SPDLOG_DEBUG("Submitting document {} for indexing", path.string());

            BS::this_thread::set_os_thread_name("Compiler");

            // indexing is conceptually very similar to compilation, but we don't collect diagnostics, don't
            // do analysis, and spend most of our effort building the document graph

            // is the initial index in progress?
            maybeUpdateIndexingProgress(path);

            // do initial CST parse
            auto tree = SyntaxTree::fromBuffer(buf, *sourceMgr);
            SPDLOG_TRACE(
                "Parsed document {}, got {} initial diagnostics", path.string(), tree->diagnostics().size());
            // this is essential so that later, we will have the parse tree associated with this current
            // document
            g_indexManager.associateParse(path, tree);

            // figure out what symbols this document provides and requires
            auto imports = ImportLocator::locateRequiredProvidedImports(tree, path);
            {
                auto lock = g_indexManager.acquireWriteLock();
                for (const auto &provided : imports.providedSymbols) {
                    g_indexManager.documentGraph->registerProvidedSymbol(path, provided);
                }
                for (const auto &required : imports.requiredSymbols) {
                    g_indexManager.documentGraph->registerRequiredSymbol(path, required);
                }
            }

            // lift to our own internal higher level representation for completion
            doLifting(path, tree);

            indexingJobsInProgress--;

            // finalise the indexing progress, if it's active
            maybeFinaliseIndexingProgress();

        } catch (const std::exception &e) {
            SPDLOG_ERROR("Caught exception in thread pool: {}", e.what());
        }
    });
}

void CompilationManager::maybeUpdateIndexingProgress(const std::filesystem::path &path) {
    if (g_indexManager.isInitialIndexInProgress) {
        // in that case, send a progress notification
        lsp::WorkDoneProgressReport report;
        report.message = "Indexing " + path.string();
        lsp::notifications::Progress::Params progress;
        progress.token = "SlingshotIndexProgress";
        progress.value = lsp::toJson(std::move(report));
        g_msgHandler->sendNotification<lsp::notifications::Progress>(std::move(progress));
    }
}

std::shared_ptr<slang::syntax::SyntaxTree> CompilationManager::doCstParse(
    const std::filesystem::path &path, const SourceBuffer &buf, DiagnosticEngine &diagEngine) {
    auto tree = SyntaxTree::fromBuffer(buf, *sourceMgr);
    SPDLOG_TRACE("Parsed document {}, got {} initial diagnostics", path.string(), tree->diagnostics().size());
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
    slangDriver.options.errorLimit = 999;
    auto options = slangDriver.createOptionBag();

    // create a compilation, so we can get further diagnostics; this will yield for us the AST,
    // whereas before we had the CST

    auto compilation = std::make_shared<Compilation>(options);
    // only initially add the document itself as a syntax tree, we'll discover the other documents later
    compilation->addSyntaxTree(tree);

    // FIXME we should take out a lock on this, probably, if we don't already have one at this time
    if (!requiredDocuments.contains(path)) {
        SPDLOG_WARN("Required documents for path {} are unknown!", path.string());
    } else {
        auto docs = requiredDocuments.at(path);
        for (const auto &doc : docs) {
            auto result = g_indexManager.retrieve(doc);
            if (result.has_value() && result != std::nullopt && (*result)->tree != nullptr) {
                SPDLOG_DEBUG("{} ---(requires)---> {}", path.string(), doc.string());
                compilation->addSyntaxTree((*result)->tree);
            } else {
                SPDLOG_WARN("Required document {} present in CompilerManager, but could not retrieve from "
                            "IndexManager",
                    doc.string());
            }
        }
    }

    // finalise it, apparently we have to call getRoot() to do this
    SPDLOG_TRACE("Finalise AST compilation");
    compilation->getRoot();
    for (const auto &diag : compilation->getAllDiagnostics()) {
        SPDLOG_TRACE("Got an AST diagnostic");
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
    for (const auto &diag : analysisMgr.getDiagnostics(&*sourceMgr)) {
        SPDLOG_DEBUG("Got an analysis diagnostic");
        // ensure the diagnostic relates to the file we're compiling
        if (diag.location.buffer() == buf.id) {
            SPDLOG_DEBUG("Issued a diagnostic in analysis");
            diagEngine.issue(diag);
        }
    }
}

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
    if (openFiles.contains(path)) {
        SPDLOG_DEBUG("Issue {} diagnostics to client for buffer {}", diagClient->getLspDiagnostics().size(),
            path.string());

        lsp::notifications::TextDocument_PublishDiagnostics::Params lspDiagMsg;
        lspDiagMsg.diagnostics = diagClient->getLspDiagnostics();
        lspDiagMsg.uri = lsp::Uri::parse("file://" + path.string());

        g_msgHandler->sendNotification<lsp::notifications::TextDocument_PublishDiagnostics>(
            std::move(lspDiagMsg));
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
        pool.detach_task([this] { performBulkCompilation(); });
    }
}

void CompilationManager::performBulkCompilation() {
    SPDLOG_INFO("Performing bulk compilation");

    auto indexLock = g_indexManager.acquireReadLock();
    auto compilerLock = g_compilerManager.acquireWriteLock();

    g_indexManager.documentGraph->finaliseOutstandingSymbols();

    // keep track of all the prior documents we've seen in our topological traversal
    std::vector<std::filesystem::path> allPriorDocs;

    // and this is why we do the topo sort, right! because now we now the exact order we need to compile all
    // the documents in!
    auto topoSort = g_indexManager.documentGraph->topologicalSort();
    if (!topoSort.has_value() || topoSort == std::nullopt) {
        SPDLOG_ERROR("Failed to topologically sort the document graph!");
        goto done; // NOLINT(cppcoreguidelines-avoid-goto): we're using it, idgaf
    }

    for (size_t i = 0; i < topoSort->size(); i++) {
        const auto &doc = topoSort->at(i);
        SPDLOG_DEBUG("({}/{}) {}", i, topoSort->size(), doc.string());

        // send progress notification
        lsp::WorkDoneProgressReport report;
        report.message = fmt::format("Bulk compiling ({}/{})", i, topoSort->size());
        lsp::notifications::Progress::Params progress;
        progress.token = "SlingshotIndexProgress";
        progress.value = lsp::toJson(std::move(report));
        g_msgHandler->sendNotification<lsp::notifications::Progress>(std::move(progress));

        requiredDocuments[doc] = allPriorDocs;

        // perform the compilation itself
        // since the doc has been indexed, we can just pull the CST out of there
        auto entry = g_indexManager.retrieve(doc);
        if (!entry.has_value() || entry == std::nullopt) {
            SPDLOG_ERROR("Document {} not in index somehow?!", doc.string());
            continue;
        }

        // auto cst = (*entry)->tree;
        // auto buf = sourceMgr->getSourceText(BufferID buffer)
        // auto ast = doAstParse(doc, const SourceBuffer &buf, DiagnosticEngine &diagEngine, const
        // std::shared_ptr<slang::syntax::SyntaxTree> &tree)

        allPriorDocs.push_back(doc);

        // FIXME I don't think we should do this allPriorDocs stuff, we should probably do a BFS-based
        // discovery
    }

    // TODO force also us to recompile all documents we have open, so we re-issue diagnostics

done:
    lsp::notifications::Progress::Params endMsg;
    endMsg.token = "SlingshotIndexProgress";
    endMsg.value = lsp::toJson(lsp::WorkDoneProgressEnd());
    g_msgHandler->sendNotification<lsp::notifications::Progress>(std::move(endMsg));
    g_indexManager.isInitialIndexInProgress = false;
}
