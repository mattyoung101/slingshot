// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <functional>
#include <nlohmann/detail/macro_scope.hpp>
#include <optional>
#include <spdlog/spdlog.h>
#include <string>
#include <utility>
#include <vector>
#include <nlohmann/json.hpp>

/// A more abstract representation of the SV language, used for completion

namespace slingshot::lang {

enum class PortDirection {
    Unknown,
    DontCare,
    Input,
    Output,
    InOut,
};

/// Represents a port in a module
class Port {
public:
    std::string name {};
    PortDirection direction = PortDirection::Unknown;

    NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Port, name, direction);
};

/// Represents a module in a document
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
        SPDLOG_DEBUG("Add parameter {} to module {}", paramName, name);
        parameters.push_back(paramName);
    }

    /// Returns the list of port directions that match the given direction. If direction is
    /// PortDirection::DontCare, then all directions will be returned. PortDirection::Unknown is treated as
    /// any valid direction.
    std::vector<std::string> getPortNames(const PortDirection &direction) {
        std::vector<std::string> out {};
        out.reserve(ports.size());
        for (const auto &port : ports) {
            // always add these
            if (port.direction == PortDirection::Unknown || direction == PortDirection::DontCare) {
                out.push_back(port.name);
                continue;
            }

            // otherwise we need to apply the filter
            if (port.direction == direction) {
                out.push_back(port.name);
            }
        }
        return out;
    }

    std::vector<Port> ports {};
    std::vector<std::string> parameters {};
    std::string name {};

    NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Module, ports, parameters, name);
};

/// Represents a document
class Document {
public:
    void startModule(const std::string &name) {
        if (currentModule != std::nullopt) {
            SPDLOG_ERROR("Starting a module when a module is already active!");
        }
        SPDLOG_DEBUG("Start module: {}", name);
        currentModule = Module(name);
    }

    void endModule() {
        if (currentModule == std::nullopt) {
            SPDLOG_ERROR("Trying to end a module, but no module is active!");
            return;
        }
        SPDLOG_DEBUG("End module: {}", currentModule->name);
        modules.push_back(*currentModule);
        currentModule = std::nullopt;
    }

    void maybeFlushModule() {
        if (currentModule != std::nullopt) {
            endModule();
        }
    }

    void doIfModuleIsActive(const std::function<void(Module &)> &routine) {
        if (currentModule != std::nullopt) {
            routine(*currentModule);
        }
    }

    std::vector<Module> modules {};

    NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Document, modules, currentModule);
private:
    std::optional<Module> currentModule = std::nullopt;
};

} // namespace slingshot::lang
