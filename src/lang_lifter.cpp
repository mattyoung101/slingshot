// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/lang_lifter.hpp"
#include "slingshot/language.hpp"
#include <slang/syntax/AllSyntax.h>

using namespace slingshot;

void LangLifterVisitor::handle(const ModuleDeclarationSyntax &syntax) {
    SPDLOG_DEBUG("Visit module");
    doc.maybeFlushModule();
    doc.startModule(std::string(syntax.header->name.valueText()));
}

void LangLifterVisitor::handle(const ImplicitAnsiPortSyntax &syntax) {
    SPDLOG_DEBUG("Visit ANSI port");
    doc.doIfModuleIsActive([&syntax](lang::Module &module) {
        // TODO get the right direction
        module.addPort(std::string(syntax.declarator->name.valueText()), lang::PortDirection::Unknown);
    });
}
