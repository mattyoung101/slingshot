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
#include <string>
#include <vector>

using namespace slingshot;

namespace {

/// Runs the block of code only if the cursor is in the node's syntax range
#define BEGIN(code)                                                                                          \
    if (containsRelaxed(cursor, syntax.sourceRange())) {                                                     \
        recommendations.clear();                                                                             \
        code;                                                                                                \
    }                                                                                                        \
    visitDefault(syntax);

#define RECOMMEND(what) recommend(what, #what)

/// Same as SourceRange::contains(), expect that it includes <= on the end position, i.e. it's inclusive
constexpr bool containsRelaxed(const SourceLocation &loc, const SourceRange &range) {
    return loc >= range.start() && loc <= range.end();
}

}; // namespace

void CompletionSyntaxVisitor::recommend(const std::vector<lsp::CompletionItem> &completions, const std::string &what) {
    addAll(recommendations, completions);
    SPDLOG_DEBUG("Recommended: {}", what);
}

void CompletionSyntaxVisitor::handle(const EventControlWithExpressionSyntax &syntax) {
    SPDLOG_DEBUG("Visit event control expr '{}' range: {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));

    BEGIN({
        // HACK we should make an AST walker for this, toString() is probably very slow
        auto parentText = syntax.parent->toString();
        if (!parentText.contains("posedge") && !parentText.contains("negedge")) {
            RECOMMEND(CompletionGenerator::generateEdge());
        }
    })
}

// FIXME this needs to be much more selective; otherwise it ends up applying to all selections
void CompletionSyntaxVisitor::handle(const ExpressionSyntax &syntax) {
    SPDLOG_DEBUG("Visit expression {} range {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));
    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateAlways());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
    });
}

void CompletionSyntaxVisitor::handle(const AnsiPortListSyntax &syntax) {
    SPDLOG_DEBUG("Visit ANSI port syntax {} range {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));

    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateInputOutput());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
    })
}

void CompletionSyntaxVisitor::handle(const ModuleDeclarationSyntax &syntax) {
    SPDLOG_DEBUG("Visit module decalaration");

    if (containsRelaxed(cursor, syntax.sourceRange())) {
        auto name = syntax.header->name.valueText();
        SPDLOG_DEBUG("Active module: {}", name);
        activeModule = name;
    }
    visitDefault(syntax);
}

std::vector<lsp::CompletionItem> CompletionManager::getCompletions(
    const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry) {
    auto tree = indexEntry->tree;

    // visit the syntax tree, based on cursor position
    auto cursor = toSlangLocation(pos, path, g_compilerManager.getSourceManager());
    SPDLOG_DEBUG("Completion cursor pos: {}", toString(cursor, g_compilerManager.getSourceManager()));
    CompletionSyntaxVisitor visitor(cursor, *indexEntry->doc);
    visitor.visit(tree->root());

    return visitor.recommendations;
}
