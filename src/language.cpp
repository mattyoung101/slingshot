// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2026 Mel Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/language.hpp"
#include "slingshot/conversions.hpp"
#include "slingshot/slingshot.hpp"
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

constexpr std::pair<lsp::Position, lsp::Uri> extractLocation(const slang::SourceLocation &location) {
    auto lock = g_compilerManager.acquireLock();
    auto sourceManager = g_compilerManager.getSourceManager();
    auto convertedLocation = toPosition(location, sourceManager);
    auto uri
        = lsp::Uri::parse(std::string("file://") + sourceManager->getFullPath(location.buffer()).string());

    return std::make_pair(convertedLocation, uri);
}

}; // namespace

Module::Module(std::string name, const slang::SourceLocation &location)
    : name(std::move(name)) {
    const auto &[convertedLocation, uri] = extractLocation(location);
    this->location = { .name = name, .pos = convertedLocation, .uri = uri };
}

void Module::addPort(const std::string &portName, PortDirection dir, const slang::SourceLocation &location) {
    const auto &[convertedLocation, uri] = extractLocation(location);
    ports.push_back(Port { .name = portName, .direction = dir });
}

void Module::addParameter(const std::string &paramName, const slang::SourceLocation &location) {
    const auto &[convertedLocation, uri] = extractLocation(location);
    parameters.insert({ .name = paramName, .pos = convertedLocation, .uri = uri });
}

void Module::addVariable(const std::string &varName, const slang::SourceLocation &location) {
    const auto &[convertedLocation, uri] = extractLocation(location);
    variables.insert({ .name = varName, .pos = convertedLocation, .uri = uri });
}
