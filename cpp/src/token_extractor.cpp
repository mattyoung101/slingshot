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
#include <type_traits>

#include "slingshot/slingshot.hpp"
#include "slingshot/token_extractor.hpp"

using namespace slingshot;
using namespace slang::syntax;
using namespace slang::ast;

struct ModuleVisitor : public ASTVisitor<ModuleVisitor, true, false> {
    template<typename T>
    void handle(const T &t) {
        if (std::is_base_of_v<Statement, T>) {
            static_cast<Statement>(t);
        }
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
    
    // /* compilation.getRoot().visit(makeVisitor(Functions funcs...), Args &&args...) */

    return result;
}
