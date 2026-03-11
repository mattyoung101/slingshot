// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/import_locator.hpp"
#include <filesystem>
#include <lsp/types.h>
#include <memory>
#include <slang/ast/Compilation.h>
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <vector>

using namespace slingshot;

// TODO listen to ScopedName

void ImportableFinderVisitor::handle(const PackageImportItemSyntax &syntax) {
    requiredSymbols.emplace_back(syntax.package.valueText());
    SPDLOG_DEBUG("{} REQUIRES {} (PackageImport)", path.string(), syntax.package.valueText());
    visitDefault(syntax);
}

void ImportableFinderVisitor::handle(const HierarchyInstantiationSyntax &syntax) {
    requiredSymbols.emplace_back(syntax.type.valueText());
    SPDLOG_DEBUG("{} REQUIRES {} (HierarchyInstantiation)", path.string(), syntax.type.valueText());
    visitDefault(syntax);
}

void ImportableFinderVisitor::handle(const IdentifierNameSyntax &syntax) {
    auto name = syntax.identifier.valueText();

    // FIXME this is completely fucking stupid we need to actually check properly if it's a pkg or not
    if (name.contains("pkg")) {
        requiredSymbols.emplace_back(name);
        SPDLOG_DEBUG("{} REQUIRES {} (HierarchyInstantiation)", path.string(), name);
        visitDefault(syntax);
    }
}

void ImportableFinderVisitor::handle(const ModuleHeaderSyntax &syntax) {
    // this apparently will also handle packages
    providedSymbols.emplace_back(syntax.name.valueText());
    SPDLOG_DEBUG("Discovered provided symbol: {}", syntax.name.valueText());
    visitDefault(syntax);
}

Imports ImportLocator::locateRequiredProvidedImports(
    const std::shared_ptr<SyntaxTree> &tree, const std::filesystem::path &path) {
    ImportableFinderVisitor visitor(path);
    visitor.visit(tree->root());

    return {
        .requiredSymbols = visitor.getRequiredSymbols(),
        .providedSymbols = visitor.getProvidedSymbols(),
    };
}
