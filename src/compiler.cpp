// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/compiler.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <lsp/types.h>
#include <slang/diagnostics/DiagnosticEngine.h>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>

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

    if (diagnostic.ranges.size() > 1) {
        SPDLOG_WARN("Cannot yet handle diagnostics.size() > 1!");
    }

    if (!diagnostic.ranges.empty()) {
        auto start = diagnostic.ranges[0].start();
        auto end = diagnostic.ranges[0].end();

        lspDiag.range.start.character = sourceManager->getColumnNumber(start);
        lspDiag.range.start.line = sourceManager->getLineNumber(start);

        lspDiag.range.end.character = sourceManager->getColumnNumber(end);
        lspDiag.range.end.line = sourceManager->getLineNumber(end);
    }

    lspDiags.push_back(lspDiag);
}

void CompilationManager::submitCompilationJob(
    const std::string &document, const std::filesystem::path &path) {
    SPDLOG_DEBUG("Submitting document {} for compilation", path.string());

    // NOTE this may leak memory
    auto buf = sourceMgr.assignText(document);
    slangBufs[path] = buf;

    pool.detach_task([buf, path, this] {
        // do compilation
        // TODO track the time it takes
        auto tree = SyntaxTree::fromBuffer(buf, sourceMgr);
        SPDLOG_DEBUG("Compiled document {}, got {} diagnostics", path.string(), tree->diagnostics().size());

        DiagnosticEngine diagEngine { sourceMgr };
        LSPDiagnosticClient::Ptr diagClient = std::make_shared<LSPDiagnosticClient>();
        diagEngine.addClient(diagClient);

        for (const auto &diag : tree->diagnostics()) {
            diagEngine.issue(diag);
        }

        g_indexManager.associateParse(path, tree);
        g_indexManager.associateDiagnostics(path, diagClient->getLspDiagnostics());
    });
}
