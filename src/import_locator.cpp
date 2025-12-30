// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/import_locator.hpp"
#include "slingshot/slingshot.hpp"
#include <lsp/types.h>
#include <memory>
#include <optional>
#include <slang/ast/Compilation.h>
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

using namespace slingshot;

void ImportableFinderVisitor::handle(const PackageImportItemSyntax &syntax) {
    requiredSymbols.emplace_back(syntax.package.valueText());
    SPDLOG_DEBUG("Discovered required symbol: {}", syntax.package.valueText());
    visitDefault(syntax);
}

void ImportableFinderVisitor::handle(const HierarchyInstantiationSyntax &syntax) {
    requiredSymbols.emplace_back(syntax.type.valueText());
    SPDLOG_DEBUG("Discovered required symbol: {}", syntax.type.valueText());
    visitDefault(syntax);
}

std::optional<std::vector<std::shared_ptr<SyntaxTree>>> ImportLocator::locateRequiredDocuments(
    const std::shared_ptr<SyntaxTree> &tree) {
    // find all the required symbols
    ImportableFinderVisitor visitor;
    visitor.visit(tree->root());

    auto requiredSymbols = visitor.getRequiredSymbols();
    SPDLOG_DEBUG("Need to find {} required symbols for the document", requiredSymbols.size());

    std::vector<SyntaxTreePtr> out;

    for (const auto &symbol : requiredSymbols) {
        // maybe it's a module?
        auto maybeModule = g_indexManager.locateDocumentForModule(symbol);
        if (maybeModule.has_value() && maybeModule != std::nullopt) {
            out.push_back(*maybeModule);
            continue;
        }

        // maybe it's a package?
        auto maybePackage = g_indexManager.locateDocumentForPackage(symbol);
        if (maybePackage.has_value() && maybePackage != std::nullopt) {
            out.push_back(*maybePackage);
            continue;
        }

        // maybe it's a typedef?

        // otherwise, everything failed: we can't resolve this item
        SPDLOG_ERROR("Unable to resolve document for symbol: {}", symbol);
        // return failure
        return std::nullopt;
    }

    // we found all the symbols, in this case
    return out;
}
