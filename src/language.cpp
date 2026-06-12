// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2026 Mel Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/language.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/slingshot.hpp"
#include <filesystem>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <nlohmann/detail/macro_scope.hpp>
#include <nlohmann/json.hpp>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <utility>

using namespace slingshot;
using namespace slingshot::lang;

namespace {

constexpr LangLocatable extractLocation(
    const std::string &name, const std::filesystem::path &path, const slang::SourceLocation &location) {
    auto lock = g_compilerManager.acquireLock();
    auto sourceManager = g_compilerManager.getSourceManager();
    auto convertedLocation = toPosition(location, sourceManager);

    return { .name = name, .pos = convertedLocation, .path = path };
}

}; // namespace

Module::Module(std::string name, const std::filesystem::path &path, const slang::SourceLocation &location)
    : name(std::move(name)) {
    this->location = extractLocation(name, path, location);
}

void Module::addPort(const std::string &portName, PortDirection dir, const slang::SourceLocation &location) {
    // clang-format off
    ports.push_back(Port {
        .name = portName,
        .direction = dir,
        .location = extractLocation(portName, this->location.path, location)
    });
    // clang-format on
}

void Module::addParameter(const std::string &paramName, const slang::SourceLocation &location) {
    parameters.insert(extractLocation(paramName, this->location.path, location));
}

void Module::addVariable(const std::string &varName, const slang::SourceLocation &location) {
    variables.insert(extractLocation(varName, this->location.path, location));
}

std::optional<LangLocatable> Module::querySymbolLocation(const std::string &symbol) const {
    for (const auto &port : ports) {
        if (port.name == symbol) {
            return port.location;
        }
    }

    for (const auto &variable : variables) {
        if (variable.name == symbol) {
            return variable;
        }
    }

    for (const auto &parameters : parameters) {
        if (parameters.name == symbol) {
            return parameters;
        }
    }

    // couldn't find it
    return std::nullopt;
}
