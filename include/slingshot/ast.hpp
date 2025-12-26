// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <memory>
#include <optional>
#include <slang/ast/Compilation.h>
#include <slang/ast/symbols/InstanceSymbols.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

namespace slingshot {

using namespace slang::ast;

/// Slingshot's wrapper around the Slang AST
class SlingAST {
public:
    SlingAST(const std::shared_ptr<Compilation> &ast)
        : ast(ast) {
            if (!ast->isFinalized()) {
                SPDLOG_ERROR("AST should be finalised before being passed to SlingAST!");
            }
    }

    /// Locates the given module in the AST if it exists
    std::optional<InstanceSymbol&> getModuleByName(const std::string &name);

private:
    std::vector<std::shared_ptr<InstanceSymbol>> getAllModules();

    /// The copy of the AST that we keep
    std::shared_ptr<Compilation> ast;
};

} // namespace slingshot
