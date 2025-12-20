// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include <exception>
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

        // FIXME I think this actually introduces a race lol
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

    // TODO I think we *DO* actually need to dedup diagnostics here as well, as well as deduping them
    // elsewhere

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

    pool.detach_task([buf, path, this] {
        try {
            SPDLOG_DEBUG("Submitting document {} for compilation", path.string());

            BS::this_thread::set_os_thread_name("Compiler");

            // do initial CST parse
            auto tree = SyntaxTree::fromBuffer(buf, *sourceMgr);
            SPDLOG_TRACE(
                "Parsed document {}, got {} initial diagnostics", path.string(), tree->diagnostics().size());
            // this is essential so that later, we will have the parse tree associated with this current
            // document
            g_indexManager.associateParse(path, tree);

            // compute diagnostics
            DiagnosticEngine diagEngine { *sourceMgr };
            // get more diagnostics
            diagEngine.setIgnoreAllNotes(false);
            diagEngine.setIgnoreAllWarnings(false);

            // this is our custom listener for diagnostics that we'll filter and report to the LSP
            LSPDiagnosticClient::Ptr diagClient = std::make_shared<LSPDiagnosticClient>(path);
            diagClient->setSourceManager(sourceMgr);
            diagEngine.addClient(diagClient);

            // first, issue syntax diagnostics we got in parsing
            for (const auto &diag : tree->diagnostics()) {
                diagEngine.issue(diag);
            }

            // try and get the default driver options, which seem to be a necessity to get diagnostics, which
            // is our only goal here atm
            driver::Driver slangDriver;
            slangDriver.addStandardArgs();
            slangDriver.options.errorLimit = 999;
            auto options = slangDriver.createOptionBag();

            // create a compilation, so we can get further diagnostics; this will yield for us the AST,
            // whereas before we had the CST
            auto trees = g_indexManager.getAllSyntaxTrees();
            SPDLOG_DEBUG("Creating AST compilation with {} syntax trees", trees.size());

            Compilation compilation;
            for (const auto &tree : trees) {
                compilation.addSyntaxTree(tree);
            }

            // finalise it, apparently we have to call getRoot() to do this
            SPDLOG_DEBUG("Finalise AST compilation");
            compilation.getRoot();
            for (const auto &diag : compilation.getAllDiagnostics()) {
                SPDLOG_DEBUG("Got an AST diagnostic");
                // ensure the diagnostic relates to the file we're compiling
                if (diag.location.buffer() == buf.id) {
                    SPDLOG_DEBUG("Issued a diagnostic in the AST");
                    diagEngine.issue(diag);
                }
            }

            // also perform analysis
            SPDLOG_DEBUG("Perform analysis");
            compilation.freeze();
            AnalysisManager analysisMgr;
            analysisMgr.analyze(compilation);
            for (const auto &diag : analysisMgr.getDiagnostics(&*sourceMgr)) {
                SPDLOG_DEBUG("Got an analysis diagnostic");
                // ensure the diagnostic relates to the file we're compiling
                if (diag.location.buffer() == buf.id) {
                    SPDLOG_DEBUG("Issued a diagnostic in analysis");
                    diagEngine.issue(diag);
                }
            }

            // lift to our own internal higher level representation for completion
            SPDLOG_DEBUG("Lifting language");
            LangLifterVisitor langLifter;
            langLifter.visit(tree->root());
            langLifter.doc.maybeFlushModule();

            SPDLOG_TRACE("Associate diagnostics and slingshot::lang document");
            g_indexManager.associateLangDoc(path, langLifter.doc);

            // publish diagnostics to the client
            // we only do this if the text document is open, to avoid extraneous errors
            if (openFiles.contains(path)) {
                SPDLOG_DEBUG("Issue {} diagnostics to client for buffer {}",
                    diagClient->getLspDiagnostics().size(), path.string());

                lsp::notifications::TextDocument_PublishDiagnostics::Params lspDiagMsg;
                lspDiagMsg.diagnostics = diagClient->getLspDiagnostics();
                lspDiagMsg.uri = lsp::Uri::parse("file://" + path.string());

                g_msgHandler->sendNotification<lsp::notifications::TextDocument_PublishDiagnostics>(
                    std::move(lspDiagMsg));
            }
        } catch (const std::exception &e) {
            SPDLOG_ERROR("Caught exception in thread pool: {}", e.what());
        }
    });
}
