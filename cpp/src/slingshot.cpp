/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#include <type_traits>
#include <iostream>
#include <set>
#include <format>
#include <slang/text/SourceManager.h>
#include <slang/ast/Compilation.h>
#include <slang/ast/ASTVisitor.h>
#include <slang/ast/SemanticModel.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <slang/syntax/SyntaxPrinter.h>
#include <slang/util/Version.h>
#include <slang/ast/symbols/PortSymbols.h>
#include <slang/ast/symbols/VariableSymbols.h>
#include <slang/diagnostics/DiagnosticEngine.h>
#include <slang/diagnostics/TextDiagnosticClient.h>
#include <slingshot/slingshot.hpp>
#include <csignal>

using namespace slang::syntax;
using namespace slang::ast;
using namespace slang::diag;
using namespace slang;

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

 CompletionResult_t slingshot_extract_completion_tokens(std::string document) {
    CompletionResult_t result;

    // first, parse the document 
    // TODO handle include path (may need a .slingshot.config or something who knows)
    auto tree = SyntaxTree::fromText(document);
    Compilation compilation;
    compilation.addSyntaxTree(tree);

    if (!compilation.getAllDiagnostics().empty()) {
        std::cout << "WARNING: Diagnostics are not empty!" << std::endl;

        // process diagnostics into text
        DiagnosticEngine engine(*compilation.getSourceManager());
        auto client = std::make_shared<TextDiagnosticClient>();
        client->showColors(false);
        engine.addClient(client);

        for (const auto &diag : compilation.getAllDiagnostics()) {
            engine.issue(diag);
        }

        std::string report = client->getString();
        std::cout << report << std::endl;
    } else {
        std::cout << "Parsed OK without any diagnostics" << std::endl;
    }
    
    // extract symbols from document
    symbols.clear();
    compilation.getRoot().visit(VariableVisitor());
    compilation.getRoot().visit(ParameterVisitor());

    for (const auto &token : symbols) {
        std::cout << token << " ";
    }
    std::cout << std::endl;

    result.tokens = symbols;
    return result;
}

std::string slingshot_get_cpp_version() {
    return SLINGSHOT_CPP_VERSION;
}

std::string slingshot_get_slang_version() {
    // semantic versioning with git hash
    return std::format("{}.{}.{}+{}", slang::VersionInfo::getMajor(), slang::VersionInfo::getMinor(),
                       slang::VersionInfo::getPatch(), slang::VersionInfo::getHash());
}
