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
#include <slang/syntax/AllSyntax.h>
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

#define RECOMMEND(what) recommend(what, #what);

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

void CompletionSyntaxVisitor::recommend(const CompletionType &type, const std::string &name) {
    recommendations.push_back(type);
    SPDLOG_DEBUG("Recommending: {}", name);
}

void CompletionSyntaxVisitor::handle(const EventControlWithExpressionSyntax &syntax) {
    SPDLOG_DEBUG("Visit event control expr {}", syntax.toString());

    BEGIN({
        if (!syntax.toString().contains("foo")) {
            RECOMMEND(CompletionType::Edge);
        }
    })
}

void CompletionSyntaxVisitor::handle(const ExpressionSyntax &syntax) {
    SPDLOG_DEBUG("Visit expression {}", syntax.toString());
    BEGIN({ RECOMMEND(CompletionType::Logic) });
}

std::vector<lsp::CompletionItem> CompletionManager::getCompletions(
    const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry) {
    auto tree = indexEntry->tree;

    // visit the syntax tree, based on cursor position
    CompletionSyntaxVisitor visitor(toSlangLocation(pos, path, g_compilerManager.getSourceManager()));
    visitor.visit(tree->root());

    // now we have the recommendation types, generate the actual items
    return CompletionGenerator::transformAll(visitor.recommendations);
}

std::vector<lsp::CompletionItem> CompletionGenerator::transformAll(
    const std::vector<CompletionType> &completions) {
    std::vector<lsp::CompletionItem> out;

    for (const auto &comp : completions) {
        switch (comp) {
            case CompletionType::Edge:
                addAll(out, generateEdge());
                break;

            case CompletionType::Logic:
                addAll(out, generateLogic());
                break;

            default:
                SPDLOG_ERROR("Unhandled completion type: {}", static_cast<int>(comp));
        }
    }

    return out;
}
