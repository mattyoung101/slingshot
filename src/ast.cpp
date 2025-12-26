// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/ast.hpp"
#include <memory>
#include <optional>
#include <slang/ast/Symbol.h>
#include <slang/ast/symbols/CompilationUnitSymbols.h>
#include <slang/ast/symbols/InstanceSymbols.h>

using namespace slingshot;
using namespace slang::ast;

std::vector<std::shared_ptr<InstanceSymbol>> SlingAST::getAllModules() {
    std::vector<std::shared_ptr<InstanceSymbol>> out;
    auto &root = ast->getRoot();
    for (auto &member : root.members()) {
        if (member.kind == SymbolKind::Instance) {
            // we have an instance: module, package, etc.
            auto &instance = member.as<InstanceSymbol>();
            if (instance.isModule()) {
                auto sym = std::make_shared<InstanceSymbol>(instance.name, instance.location, instance.body);
            }
        }
    }
    return out;
}

// std::optional<InstanceSymbol> SlingAST::getModuleByName(const std::string &name) {
//     const auto &root = ast->getRoot();
//     for (const auto &member : root.members()) {
//         if (member.kind == SymbolKind::Instance) {
//             // we have an instance: module, package, etc.
//             const auto &instance = member.as<InstanceSymbol>();
//             if (instance.name == name) {
//                 return instance;
//             }
//         }
//     }
// }
