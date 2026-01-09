// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/completion.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/language.hpp"
#include "slingshot/slingshot.hpp"
#include <algorithm>
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <slang/diagnostics/DiagnosticEngine.h>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxKind.h>
#include <slang/syntax/SyntaxNode.h>
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

inline bool containsInDirectHierarchy(const SyntaxNode &node, const std::vector<SyntaxKind> &kinds) {
    SyntaxNode *parent = node.parent;
    while (parent != nullptr) {
        // NOTE this should use std::ranges::contains(), but we're still choosing to support Ubuntu 22.04 as
        // a compile target for now, and that OS doesn't correctly support C++23 because it uses an old GNU
        // libstdc++ I think.
        // NOLINTNEXTLINE
        if (std::find(kinds.begin(), kinds.end(), parent->kind) != kinds.end()) {
            return true;
        }
        // get the parent's parent
        parent = parent->parent;
    }
    return false;
}

const std::vector<SyntaxKind> ALWAYS_BLOCK = { SyntaxKind::AlwaysCombBlock, SyntaxKind::AlwaysFFBlock,
    SyntaxKind::AlwaysLatchBlock, SyntaxKind::AlwaysBlock };

}; // namespace

void CompletionSyntaxVisitor::recommend(
    const std::vector<lsp::CompletionItem> &completions, const std::string &what) {
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
        } else {
            // now that we've typed "posedge"/"negedge"; we can recommend variables
            // making sure we only recommend either Input or InOut ports
            RECOMMEND(CompletionGenerator::generateVariableSameModuleFilter(
                activeModule, doc, lang::PortDirection::Input));
            RECOMMEND(CompletionGenerator::generateVariableSameModuleFilter(
                activeModule, doc, lang::PortDirection::InOut));
        }
    })
}

void CompletionSyntaxVisitor::handle(const ExpressionStatementSyntax &syntax) {
    SPDLOG_TRACE("Visit expression {}", syntax.toString(),
        toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));
    SPDLOG_TRACE("Type of the expression parent is: {}", toString(syntax.parent->kind));
    BEGIN({
        // TODO determine if we are on LHS or RHS and change what we recommend

        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
        RECOMMEND(CompletionGenerator::generateIf());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));

        if (!containsInDirectHierarchy(syntax, ALWAYS_BLOCK)) {
            RECOMMEND(CompletionGenerator::generateAlways());
        }
    });
}

void CompletionSyntaxVisitor::handle(const AnsiPortListSyntax &syntax) {
    SPDLOG_TRACE("Visit ANSI port syntax {}", syntax.toString());

    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateInputOutput());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
    })
}

// NOTE: the parser detects typing in a module as a DataDeclaration a lot of the time
void CompletionSyntaxVisitor::handle(const DataDeclarationSyntax &syntax) {
    SPDLOG_TRACE("Visit data declaration syntax: {}", syntax.toString());

    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateIf());
        RECOMMEND(CompletionGenerator::generateAlways());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
        // don't recommend system tasks because we're more or less on the LHS of something
    })
}

// TODO continuous assign

void CompletionSyntaxVisitor::handle(const ModuleDeclarationSyntax &syntax) {
    SPDLOG_TRACE("Visit module decalaration");

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
