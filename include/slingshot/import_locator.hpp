// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <lsp/types.h>
#include <memory>
#include <slang/ast/Compilation.h>
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

namespace slingshot {

using namespace slang::syntax;

/// Finds things in a document that are "importable" from other documents:
/// - Packages
/// - Modules
/// - Enums/typedefs
class ImportableFinderVisitor : public SyntaxVisitor<ImportableFinderVisitor> {
public:
    void handle(const PackageImportItemSyntax &syntax);

    void handle(const HierarchyInstantiationSyntax &syntax);

    auto getRequiredSymbols() {
        return requiredSymbols;
    }

private:
    std::vector<std::string> requiredSymbols;
};

class ImportLocator {
public:
    /// Uses the @ref ImportableFinderVisitor and index to locate required documents to compile the given
    /// document
    /// Returns std::nullopt if we couldn't find any of the symbols required
    static std::optional<std::vector<std::shared_ptr<SyntaxTree>>> locateRequiredDocuments(
        const std::shared_ptr<SyntaxTree> &tree);
};

} // namespace slingshot
