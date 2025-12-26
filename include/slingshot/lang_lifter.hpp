// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "slingshot/language.hpp"
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <spdlog/spdlog.h>

/// Walks the Slang CST to lift that document into our internal slingshot::lang::Document, which can then be
/// used for completion

namespace slingshot {

using namespace slang::syntax;

class LangLifterVisitor : public SyntaxVisitor<LangLifterVisitor> {
public:
    void handle(const ModuleHeaderSyntax &syntax);
    void handle(const NetPortHeaderSyntax &syntax);
    void handle(const DeclaratorSyntax &syntax);
    void handle(const ImplicitAnsiPortSyntax &syntax);

    lang::Document doc;
};

}; // namespace slingshot
