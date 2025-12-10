// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/completion.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <slang/diagnostics/DiagnosticEngine.h>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>

using namespace slingshot;

/// Runs the block of code only if the cursor is in the node's syntax range
#define BEGIN(code)                                                                                          \
    if (syntax.sourceRange().contains(cursor)) {                                                             \
        recommendations.clear();                                                                             \
        code;                                                                                                \
    }

#define RECOMMEND(what) recommendations.push_back(what);

std::vector<lsp::CompletionItem> CompletionGenerator::generateLogic() {
    return {
        lsp::CompletionItem {
            .label = "logic",
            .kind = lsp::CompletionItemKind::Keyword,
        },
        lsp::CompletionItem {
            .label = "wire",
            .kind = lsp::CompletionItemKind::Keyword,
        },
        lsp::CompletionItem {
            .label = "reg",
            .kind = lsp::CompletionItemKind::Keyword,
        },
    };
}

std::vector<lsp::CompletionItem> CompletionGenerator::generateEdge() {
    return {
        lsp::CompletionItem {
            .label = "posedge",
            .kind = lsp::CompletionItemKind::Keyword,
        },
        lsp::CompletionItem {
            .label = "negedge",
            .kind = lsp::CompletionItemKind::Keyword,
        },
    };
}

void CompletionSyntaxVisitor::handle(const EventControlWithExpressionSyntax &syntax) {
    SPDLOG_INFO("Visit event control expr!");
    if (syntax.sourceRange().contains(cursor)) {
        SPDLOG_INFO("IT CONTAINS THE CURSOR :fire:");
    }

    // TODO only do this if there is no posedge or negedge on the line already
    BEGIN({ RECOMMEND(CompletionType::Edge); })
}

std::vector<lsp::CompletionItem> CompletionManager::getCompletions(
    const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry) {
    auto tree = indexEntry->tree;

    CompletionSyntaxVisitor visitor(toSlangLocation(pos, path, g_compilerManager.getSourceManager()));
    visitor.visit(tree->root());

    return {};
}
