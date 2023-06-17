/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#include <slang/text/SourceManager.h>
#include <slang/ast/Compilation.h>
#include <slang/ast/ASTVisitor.h>
#include <slang/ast/SemanticModel.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <slang/syntax/SyntaxPrinter.h>
#include <type_traits>
#include <iostream>
#include <set>

#include "slang/ast/symbols/PortSymbols.h"
#include "slang/ast/symbols/VariableSymbols.h"
#include "slingshot/slingshot.hpp"
#include "slingshot/token_extractor.hpp"

using namespace slingshot;
using namespace slang::syntax;
using namespace slang::ast;

static std::vector<std::string> symbols;

/// Visits SV "variables" (wires, regs, logics). Currently, in Slang, this module also picks up module ports.
struct VariableVisitor : public ASTVisitor<VariableVisitor, true, false> {
    void handle(const VariableSymbol &t) {
        symbols.emplace_back(t.name);
        visitDefault(t);
    }
};

/// Visit SV parameters
struct ParameterVisitor : public ASTVisitor<ParameterVisitor, true, false> {
    void handle(const ParameterSymbol &t) {
        symbols.emplace_back(t.name);
        visitDefault(t);
    }
};

CompletionResult CompletionTokenExtractor::extractTokens(std::string document) {
    CompletionResult result;

    // first, parse the document 
    // TODO consider sharing compilations for performance?
    // TODO handle include directories
    // TODO probably do compilation at a step further above and pass the compilation around
    auto tree = SyntaxTree::fromText(document);
    Compilation compilation;
    compilation.addSyntaxTree(tree);
    
    // TODO handle this elsewhere
    if (!compilation.getAllDiagnostics().empty()) {
        std::cout<< "WARNING: Diagnostics are not empty!" << std::endl;
    } else {
        std::cout << "Parsed OK without any diagnostics" << std::endl;
    }
    
    symbols.clear();
    compilation.getRoot().visit(VariableVisitor());
    compilation.getRoot().visit(ParameterVisitor());
    
    /* std::cout << SyntaxPrinter::printFile(*tree) << std::endl; */

    for (const auto &token : symbols) {
        std::cout << token << " ";
    }
    std::cout << std::endl;

    return result;
}
