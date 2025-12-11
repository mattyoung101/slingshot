// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once

// A more abstract representation of the SV language, used for completion

#include <spdlog/spdlog.h>
#include <string>
#include <utility>
#include <vector>
namespace slingshot::lang {

enum class PortDirection {
    Unknown,
    Input,
    Output,
    InOut,
};

class Port {
public:
    std::string name {};
    PortDirection direction = PortDirection::Unknown;
};

class Module {
public:
    Module(std::string name)
        : name(std::move(name)) {
    }

    void addPort(const std::string &portName, PortDirection dir) {
        SPDLOG_DEBUG("Add port {} to module {}", portName, name);
        ports.push_back(Port { .name = portName, .direction = dir });
    }

    void addParameter(const std::string &paramName) {
        parameters.push_back(paramName);
    }

    std::vector<Port> ports {};
    std::vector<std::string> parameters {};
    std::string name {};
};

class Document {
public:
    std::vector<Module> modules {};
};

} // namespace slingshot::lang
