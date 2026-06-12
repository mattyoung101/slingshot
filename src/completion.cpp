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
#include <exception>
#include <filesystem>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <optional>
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

/// Returns true iff any one of the nodes of type "kind" exists in the direct parental hierarchy of the given
/// node "node"
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

/// Any type of always block
const std::vector<SyntaxKind> ALWAYS_BLOCK = { SyntaxKind::AlwaysCombBlock, SyntaxKind::AlwaysFFBlock,
    SyntaxKind::AlwaysLatchBlock, SyntaxKind::AlwaysBlock };

}; // namespace

void CompletionSyntaxVisitor::recommend(
    const std::vector<lsp::CompletionItem> &completions, const std::string &what) {
    addAll(recommendations, completions);
    SPDLOG_DEBUG("Recommended: {}", what);
}

void CompletionSyntaxVisitor::handle(const EventControlWithExpressionSyntax &syntax) {
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

        SPDLOG_DEBUG("Complete event control expr '{}' range: {}", syntax.toString(),
            toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));
    })
}

void CompletionSyntaxVisitor::handle(const ExpressionStatementSyntax &syntax) {
    BEGIN({
        // TODO determine if we are on LHS or RHS and change what we recommend

        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
        RECOMMEND(CompletionGenerator::generateIf());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
        RECOMMEND(CompletionGenerator::generateStandardMacros());

        if (!containsInDirectHierarchy(syntax, ALWAYS_BLOCK)) {
            RECOMMEND(CompletionGenerator::generateAlways());
        }

        SPDLOG_TRACE("Complete expression statement: {}", syntax.toString(),
            toString(syntax.sourceRange(), g_compilerManager.getSourceManager()));
        // SPDLOG_TRACE("Type of the expression parent is: {}", toString(syntax.parent->kind));
    });
}

void CompletionSyntaxVisitor::handle(const AnsiPortListSyntax &syntax) {
    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateInputOutput());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
        SPDLOG_TRACE("Complete ANSI port syntax {}", syntax.toString());
    })
}

// NOTE: the parser detects typing in a module as a DataDeclaration a lot of the time
void CompletionSyntaxVisitor::handle(const DataDeclarationSyntax &syntax) {
    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateIf());
        RECOMMEND(CompletionGenerator::generateAlways());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
        RECOMMEND(CompletionGenerator::generateStandardMacros());

        if (!containsInDirectHierarchy(syntax, ALWAYS_BLOCK)) {
            RECOMMEND(CompletionGenerator::generateModuleInstantiations());
        }

        // don't recommend system tasks because we're more or less on the LHS of something
        //
        SPDLOG_TRACE("Complete data declaration syntax: {}", syntax.toString());
    })
}

// TODO continuous assign

void CompletionSyntaxVisitor::handle(const ModuleDeclarationSyntax &syntax) {
    if (containsRelaxed(cursor, syntax.sourceRange())) {
        auto name = syntax.header->name.valueText();
        SPDLOG_DEBUG("Active module: {}", name);
        activeModule = name;
        SPDLOG_TRACE("Complete module decalaration: {}", syntax.toString());
    }
    visitDefault(syntax);
}

void CompletionSyntaxVisitor::handle(const ConditionalPredicateSyntax &syntax) {
    BEGIN({
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
        SPDLOG_TRACE("Complete conditional predicate syntax: {}", syntax.toString());
    })
}

void CompletionSyntaxVisitor::handle(const HierarchyInstantiationSyntax &syntax) {
    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateIf());
        RECOMMEND(CompletionGenerator::generateAlways());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
        RECOMMEND(CompletionGenerator::generateStandardMacros());

        if (!containsInDirectHierarchy(syntax, ALWAYS_BLOCK)) {
            RECOMMEND(CompletionGenerator::generateModuleInstantiations());
        }
        SPDLOG_TRACE("Complete hierarchy instantiation syntax syntax: {}", syntax.toString());
    })
}

void CompletionSyntaxVisitor::handle(const SimpleSequenceExprSyntax &syntax) {
    BEGIN({
        if (syntax.expr->kind == SyntaxKind::StringLiteralExpression) {
            // recommend nothing, we're in a string
            RECOMMEND({ });
        } else {
            RECOMMEND(CompletionGenerator::generateLogic());
            RECOMMEND(CompletionGenerator::generateSystemTasks());
            RECOMMEND(CompletionGenerator::generateIf());
            RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
            RECOMMEND(CompletionGenerator::generateStandardMacros());

            if (!containsInDirectHierarchy(syntax, ALWAYS_BLOCK)) {
                RECOMMEND(CompletionGenerator::generateModuleInstantiations());
                RECOMMEND(CompletionGenerator::generateAlways());
            }
        }

        SPDLOG_TRACE("Complete SimpleSequenceExpr syntax: {}", syntax.toString());
    })
}

void CompletionSyntaxVisitor::handle(const IdentifierNameSyntax &syntax) {
    BEGIN({
        RECOMMEND(CompletionGenerator::generateLogic());
        RECOMMEND(CompletionGenerator::generateSystemTasks());
        RECOMMEND(CompletionGenerator::generateIf());
        RECOMMEND(CompletionGenerator::generateVariableSameModule(activeModule, doc));
        RECOMMEND(CompletionGenerator::generateStandardMacros());

        if (!containsInDirectHierarchy(syntax, ALWAYS_BLOCK)) {
            RECOMMEND(CompletionGenerator::generateModuleInstantiations());
            RECOMMEND(CompletionGenerator::generateAlways());
        }
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////

constexpr bool recListContains(
    const std::vector<lsp::CompletionItem> &recs, const lsp::CompletionItem &target) {
    auto targetKind = target.kind.value_or(lsp::CompletionItemKind::Unit).value();
    for (const auto &rec : recs) {
        auto recKind = rec.kind.value_or(lsp::CompletionItemKind::Unit).value();
        if (recKind == targetKind && rec.label == target.label) {
            return true;
        }
    }
    return false;
}

std::vector<lsp::CompletionItem> CompletionManager::getCompletions(
    const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry) {
    auto tree = indexEntry->tree;

    // visit the syntax tree, based on cursor position
    auto cursor = toSlangLocation(pos, path, g_compilerManager.getSourceManager());
    SPDLOG_DEBUG("Completion cursor pos: {}", toString(cursor, g_compilerManager.getSourceManager()));

    if (indexEntry->doc == std::nullopt) {
        // document not available
        return { };
    }

    // determine if we are in a comment
    try {
        auto lines = split_string(indexEntry->contents, "\n");
        const auto &line = lines.at(pos.line);

        SPDLOG_DEBUG("Completion line: {}", line);

        auto indexOfSlashSlash = line.find("//");
        SPDLOG_DEBUG("Comment exists at {}, we are {}", indexOfSlashSlash, pos.character);
        if (indexOfSlashSlash != std::string::npos && pos.character >= indexOfSlashSlash) {
            return { };
        }
    } catch (const std::exception &e) {
        SPDLOG_ERROR("Failed to get contents of line idx {}, why: {}", pos.line, e.what());
        return { };
    }

    CompletionSyntaxVisitor visitor(cursor, *indexEntry->doc);
    visitor.visit(tree->root());

    // remove duplicated items
    std::vector<lsp::CompletionItem> deduped;
    // FIXME this is something like O(n^2); we should use ankerl::unordered_dense::set (see git stash); it
    // wasn't compiling after much effort due to std::equal_to shenanigans
    for (const auto &rec : visitor.recommendations) {
        if (!recListContains(deduped, rec)) {
            deduped.push_back(rec);
        }
    }

    return deduped;
}
