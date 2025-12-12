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
#include <vector>

using namespace slingshot;

namespace {

/// Runs the block of code only if the cursor is in the node's syntax range
#define BEGIN(code)                                                                                          \
    if (containsRelaxed(cursor, syntax.sourceRange())) {                                                     \
        recommendations.clear();                                                                             \
        code;                                                                                                \
    }

#define RECOMMEND(what) recommend(what, #what);

/// Same as SourceRange::contains(), expect that it includes <= on the end position, i.e. it's inclusive
constexpr bool containsRelaxed(const SourceLocation &loc, const SourceRange &range) {
    return loc >= range.start() && loc <= range.end();
}

}; // namespace

void CompletionSyntaxVisitor::recommend(const CompletionType &type, const std::string &name) {
    recommendations.push_back(type);
    SPDLOG_DEBUG("Recommending: {}", name);
}

void CompletionSyntaxVisitor::handle(const EventControlWithExpressionSyntax &syntax) {
    SPDLOG_DEBUG("Visit event control expr '{}' range: {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));

    BEGIN({
        // HACK we should make an AST walker for this, toString() is probably very slow
        auto parentText = syntax.parent->toString();
        if (!parentText.contains("posedge") && !parentText.contains("negedge")) {
            RECOMMEND(CompletionType::Edge);
        }
    })
}

void CompletionSyntaxVisitor::handle(const ExpressionSyntax &syntax) {
    SPDLOG_DEBUG("Visit expression {} range {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));
    BEGIN({
        RECOMMEND(CompletionType::Logic);
        RECOMMEND(CompletionType::Always);
        RECOMMEND(CompletionType::SystemTask);
    });
}

void CompletionSyntaxVisitor::handle(const AnsiPortListSyntax &syntax) {
    SPDLOG_DEBUG("Visit ANSI port syntax {} range {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));

    BEGIN({
        RECOMMEND(CompletionType::Logic);
        RECOMMEND(CompletionType::SystemTask);
        RECOMMEND(CompletionType::InputOutput);
    })
}

std::vector<lsp::CompletionItem> CompletionManager::getCompletions(
    const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry) {
    auto tree = indexEntry->tree;

    // visit the syntax tree, based on cursor position
    auto cursor = toSlangLocation(pos, path, g_compilerManager.getSourceManager());
    SPDLOG_DEBUG("Completion cursor pos: {}", toString(cursor, g_compilerManager.getSourceManager()));
    CompletionSyntaxVisitor visitor(cursor);
    visitor.visit(tree->root());

    // now we have the recommendation types, generate the actual items
    return CompletionGenerator::transformAll(visitor.recommendations);
}
