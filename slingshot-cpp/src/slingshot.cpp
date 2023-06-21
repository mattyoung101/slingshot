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
#include <slingshot/slingshot.h>
#include <csignal>
#include <cstdlib>

using namespace slang::syntax;
using namespace slang::ast;
using namespace slang::diag;
using namespace slang;

// string trim, source: https://stackoverflow.com/a/217605/5007892
// trim from start (in place)
static constexpr inline void ltrim(std::string &s) {
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
        return !std::isspace(ch);
    }));
}

// trim from end (in place)
static constexpr inline void rtrim(std::string &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
        return !std::isspace(ch);
    }).base(), s.end());
}

// trim from both ends (in place)
static constexpr inline void trim(std::string &s) {
    rtrim(s);
    ltrim(s);
}

/// List of symbols that we extracted from the document on the current invocation
static std::vector<std::string> symbols;

/// Visits SV "variables" (wires, regs, logics). Currently, in Slang, this visitor also picks up module ports.
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

 CompletionResult_t slingshot_extract_completion_tokens(const char *document, bool debug) {
    CompletionResult_t result;

    // first, parse the document 
    // TODO handle include path (may need a .slingshot.config or something who knows)
    auto tree = SyntaxTree::fromText(document);
    Compilation compilation;
    compilation.addSyntaxTree(tree);

    std::vector<Diagnostic_t> localDiagnostics;

    if (!compilation.getAllDiagnostics().empty()) {
        if (debug) std::cout << "Diagnostics are NOT empty!" << std::endl;

        // process diagnostics into text
        // this approach currently relies on using Slang's DiagnosticEngine, although we are really only using
        // it to print the diagnostic message to a string. we should consider doing that ourselves if performance
        // in this section becomes an issue.
        // note that we only instantiate the DiagnosticEngine if any diagnostics are detected, so there is a
        // fast-path for working code.
        // TODO come to think of it, we probably don't want to check for diagnostics in the autocomplete code
        // ...only check for diagnostics when an actual edit has gone through (like hitting esc in vim)
        DiagnosticEngine engine(*compilation.getSourceManager());
        auto client = std::make_shared<TextDiagnosticClient>();
        client->showColors(false);
        client->showLocation(false);
        client->showSourceLine(false);
        engine.addClient(client);

        for (const auto &diag : compilation.getAllDiagnostics()) {
            engine.issue(diag);

            auto report = client->getString();
            // messages include a newline that needs removing
            trim(report);
            auto line = compilation.getSourceManager()->getLineNumber(diag.location);
            auto offset = compilation.getSourceManager()->getColumnNumber(diag.location);
            if (debug) {
                std::cout << std::format("Diagnostic. Line: {}, Offset: {}, Message: {}", line, offset, report) 
                << std::endl;
            }

            // add the diagnostic to the output result that we'll return to Rust
            localDiagnostics.emplace_back(Diagnostic_t{strdup(report.c_str()), line, offset});
            
            // reset the buffer for the next diagnostic
            client->clear();
        }
    } else {
        if (debug) std::cout << "Parsed OK without any diagnostics" << std::endl;
    }
    
    // extract symbols from document
    symbols.clear();
    compilation.getRoot().visit(VariableVisitor());
    compilation.getRoot().visit(ParameterVisitor());
    
    if (debug) {
        std::cout << "Tokens:" << std::endl;
        for (const auto &token : symbols) {
            std::cout << token << " ";
        }
        std::cout << std::endl;
    }

    // turn our C++ vectors into C arrays
    result.numTokens = symbols.size();
    result.numDiagnostics = localDiagnostics.size();

    result.tokens = static_cast<char**>(calloc(result.numTokens, sizeof(char*)));
    result.diagnostics = static_cast<Diagnostic_t*>(calloc(result.numDiagnostics, sizeof(Diagnostic_t)));

    for (size_t i = 0; i < symbols.size(); i++) {
        result.tokens[i] = strdup(symbols[i].c_str());
    }

    for (size_t i = 0; i < localDiagnostics.size(); i++) {
        result.diagnostics[i] = localDiagnostics[i];
    }

    return result;
}

void slingshot_free_completion(CompletionResult_t result) {
    for (size_t i = 0; i < result.numTokens; i++) {
        free(result.tokens[i]);
    }
    free(result.tokens);

    for (size_t i = 0; i < result.numDiagnostics; i++) {
        free(result.diagnostics[i].message);
    }
    free(result.diagnostics);
}

char *slingshot_get_cpp_version() {
    return strdup(SLINGSHOT_CPP_VERSION);
}

char *slingshot_get_slang_version() {
    // semantic versioning with git hash
    return strdup(std::format("{}.{}.{}+{}", slang::VersionInfo::getMajor(), slang::VersionInfo::getMinor(),
                       slang::VersionInfo::getPatch(), slang::VersionInfo::getHash()).c_str());
}

void slingshot_free_str(char *str) {
    free(str);
}