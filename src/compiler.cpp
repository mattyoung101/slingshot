// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/compiler.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <slang/diagnostics/DiagnosticEngine.h>
#include <slang/diagnostics/Diagnostics.h>
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

void LSPDiagnosticClient::report(const ReportedDiagnostic &diagnostic) {
    SPDLOG_DEBUG("LSPDiagnosticClient::report Received a diagnostic");

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

    // auto uri = mainLoc->uri;
    // m_diagnostics[uri].push_back(lsp::Diagnostic {
    //     .range = mainLoc->range,
    //     .severity = convertSeverity(diag.severity),
    //     .message = std::string { diag.formattedMessage },
    //     .relatedInformation = related.empty() ? std::nullopt : std::optional { related },
    // });
    //
    // // Add diag code link if any
    // std::string_view optionName = engine->getOptionName(diag.originalDiagnostic.code);
    // if (!optionName.empty()) {
    //     m_diagnostics[uri].back().code = { std::string(optionName) };
    //     m_diagnostics[uri].back().codeDescription = lsp::CodeDescription { .href
    //         = URI::fromWeb("sv-lang.com/warning-ref.html#" + std::string(optionName)) };
    // }

    lspDiag.range = mainLoc->range;

    lspDiags.push_back(lspDiag);
}

void CompilationManager::submitCompilationJob(
    const std::string &document, const std::filesystem::path &path) {
    SPDLOG_DEBUG("Submitting document {} for compilation", path.string());

    // NOTE this may leak memory
    auto buf = sourceMgr->assignText(document);

    pool.detach_task([buf, path, this] {
        // do compilation
        // TODO track the time it takes
        auto tree = SyntaxTree::fromBuffer(buf, *sourceMgr);
        SPDLOG_DEBUG("Compiled document {}, got {} diagnostics", path.string(), tree->diagnostics().size());

        DiagnosticEngine diagEngine { *sourceMgr };
        LSPDiagnosticClient::Ptr diagClient = std::make_shared<LSPDiagnosticClient>();
        diagClient->setSourceManager(sourceMgr);
        diagEngine.addClient(diagClient);

        for (const auto &diag : tree->diagnostics()) {
            diagEngine.issue(diag);
        }

        g_indexManager.associateParse(path, tree);
        g_indexManager.associateDiagnostics(path, diagClient->getLspDiagnostics());

        // publish diagnostics to the client
        // we only do this if the text document is open, to avoid extraneous errors
        if (openFiles.contains(path)) {
            lsp::notifications::TextDocument_PublishDiagnostics::Params lspDiagMsg;
            lspDiagMsg.diagnostics = diagClient->getLspDiagnostics();
            lspDiagMsg.uri = lsp::Uri::parse("file://" + path.string());
            g_msgHandler->sendNotification<lsp::notifications::TextDocument_PublishDiagnostics>(
                std::move(lspDiagMsg));
        }
    });
}
