// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/lang_lifter.hpp"
#include "slingshot/language.hpp"
#include <slang/parsing/TokenKind.h>
#include <slang/syntax/AllSyntax.h>
#include <spdlog/spdlog.h>

using namespace slingshot;

void LangLifterVisitor::handle(const ModuleHeaderSyntax &syntax) {
    SPDLOG_TRACE("Visit module header");
    doc.maybeFlushModule();
    doc.startModule(std::string(syntax.name.valueText()));
    visitDefault(syntax);
}

void LangLifterVisitor::handle(const NetPortHeaderSyntax &syntax) {
    SPDLOG_TRACE("Visit net port header: {}", syntax.toString());
    doc.doIfModuleIsActive([&syntax](lang::Module &module) {
        // first, find the port direction
        lang::PortDirection direction = lang::PortDirection::Unknown;
        if (syntax.direction.kind == slang::parsing::TokenKind::InputKeyword) {
            direction = lang::PortDirection::Input;
        } else if (syntax.direction.kind == slang::parsing::TokenKind::OutputKeyword) {
            direction = lang::PortDirection::Output;
        } else if (syntax.direction.kind == slang::parsing::TokenKind::InOutKeyword) {
            direction = lang::PortDirection::InOut;
        } else {
            SPDLOG_WARN("Unknown port direction from Slang: {}", syntax.direction.toString());
        }

        // next, we need to find the name of the port; locate its header first
        auto *implicitAnsiPort = syntax.parent->as_if<ImplicitAnsiPortSyntax>();
        if (implicitAnsiPort == nullptr) {
            SPDLOG_WARN("Unable to determine port name for port: {}", syntax.toString());
            return;
        }
        auto portName = implicitAnsiPort->declarator->name.valueText();

        module.addPort(std::string(portName), direction);
    });
    visitDefault(syntax);
}

void LangLifterVisitor::handle(const ImplicitAnsiPortSyntax &syntax) {
    SPDLOG_TRACE("Visit implicit ANSI port: {}", syntax.toString());
    doc.doIfModuleIsActive([&syntax](lang::Module &module) {
        // first, find the port direction
        lang::PortDirection direction = lang::PortDirection::Unknown;

        auto *header = syntax.header->as_if<VariablePortHeaderSyntax>();
        if (header == nullptr) {
            SPDLOG_WARN("Could not get header as a VariablePortHeader for syntax: {}", syntax.toString());
            return;
        }

        if (header->direction.kind == slang::parsing::TokenKind::InputKeyword) {
            direction = lang::PortDirection::Input;
        } else if (header->direction.kind == slang::parsing::TokenKind::OutputKeyword) {
            direction = lang::PortDirection::Output;
        } else if (header->direction.kind == slang::parsing::TokenKind::InOutKeyword) {
            direction = lang::PortDirection::InOut;
        } else {
            SPDLOG_WARN("Unknown port direction from Slang: {}", header->direction.toString());
        }

        auto portName = syntax.declarator->name.valueText();

        module.addPort(std::string(portName), direction);
    });
    visitDefault(syntax);
}

void LangLifterVisitor::handle(const DeclaratorSyntax &syntax) {
    SPDLOG_TRACE("Visit declarator: {}", syntax.toString());
    doc.doIfModuleIsActive([&syntax](lang::Module &module) {
        auto name = std::string(syntax.name.valueText());

        // first, check if it's not already a port
        for (const auto &port : module.ports) {
            if (port.name == name) {
                return;
            }
        }
        module.addVariable(name);
    });
    visitDefault(syntax);
}

void LangLifterVisitor::handle(const ParameterDeclarationSyntax &syntax) {
    SPDLOG_TRACE("Handle param declaration: {}", syntax.toString());
    doc.doIfModuleIsActive([&syntax](lang::Module &module) {
        if (syntax.declarators.size() != 1) {
            SPDLOG_WARN("Parameter declarator {} does not contain exactly 1 declaration?!",
                syntax.declarators.toString());
            return;
        }

        auto name = syntax.declarators[0]->name.valueText();
        module.addParameter(std::string(name));
    });
    visitDefault(syntax);
}
