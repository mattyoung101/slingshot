// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 Mel Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <ankerl/unordered_dense.h>
#include <functional>
#include <lsp/types.h>
#include <lsp/uri.h>
#include <nlohmann/detail/macro_scope.hpp>
#include <nlohmann/json.hpp>
#include <optional>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <string_view>
#include <unordered_set>
#include <vector>

/// A more abstract representation of the SV language, used for completion

namespace lsp {
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Uri, data());
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Position, character, line);
};

namespace slingshot::lang {

/// Represents a generic token with a name and a location
class LangLocatable {
public:
    std::string name { };
    lsp::Position pos { };
    lsp::Uri uri { }; // document URI
                      //
    bool operator==(const LangLocatable &l) const {
        return l.name == name && l.pos.character == pos.character && l.pos.line == pos.line
            && l.uri.data() == uri.data();
    }
};

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(LangLocatable, name, pos, uri);

} // namespace slingshot::lang

namespace std {
using namespace slingshot::lang;
template <>
struct hash<LangLocatable> {
    std::size_t operator()(const LangLocatable &l) const noexcept {
        uint64_t hash = 0xBEEF;

        auto name = ankerl::unordered_dense::hash<std::string> { }(l.name);
        auto posLine = ankerl::unordered_dense::hash<lsp::uint> { }(l.pos.line);
        auto posChar = ankerl::unordered_dense::hash<lsp::uint> { }(l.pos.character);
        auto uri = ankerl::unordered_dense::hash<std::string_view> { }(l.uri.data());

        hash = ankerl::unordered_dense::detail::wyhash::mix(hash, name);
        hash = ankerl::unordered_dense::detail::wyhash::mix(hash, posLine);
        hash = ankerl::unordered_dense::detail::wyhash::mix(hash, posChar);
        hash = ankerl::unordered_dense::detail::wyhash::mix(hash, uri);

        return hash;
    }
};
}; // namespace std

namespace slingshot::lang {

enum class PortDirection {
    Unknown,
    DontCare,
    Input,
    Output,
    InOut,
};

#define ENUM_ENTRY(name) { PortDirection::name, #name }

NLOHMANN_JSON_SERIALIZE_ENUM(PortDirection,
    {
        ENUM_ENTRY(Unknown),
        ENUM_ENTRY(DontCare),
        ENUM_ENTRY(Input),
        ENUM_ENTRY(Output),
        ENUM_ENTRY(InOut),
    });

/// Represents a port in a module
class Port {
public:
    std::string name { };
    PortDirection direction = PortDirection::Unknown;
    LangLocatable location {};
};

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Port, name, direction, location);

/// Represents a module in a document
class Module {
public:
    Module(std::string name, const slang::SourceLocation &location);

    void addPort(const std::string &portName, PortDirection dir, const slang::SourceLocation &location);

    void addParameter(const std::string &paramName, const slang::SourceLocation &location);

    void addVariable(const std::string &varName, const slang::SourceLocation &location);

    /// Returns the list of port directions that match the given direction. If direction is
    /// PortDirection::DontCare, then all directions will be returned. PortDirection::Unknown is treated as
    /// any valid direction.
    std::vector<std::string> getPortNames(const PortDirection &direction) {
        std::vector<std::string> out { };
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

    std::vector<Port> ports { };
    // these are left as std::unordered_sets for the benefit of nlohmann::json
    std::unordered_set<LangLocatable> variables { };
    std::unordered_set<LangLocatable> parameters { };
    std::string name { };
    LangLocatable location { };
};

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Module, ports, parameters, name, variables);

/// Represents a document
class Document {
public:
    void startModule(const std::string &name, const slang::SourceLocation &location) {
        if (currentModule != std::nullopt) {
            SPDLOG_ERROR("Starting a module when a module is already active!");
        }
        // SPDLOG_TRACE("Start module: {}", name);
        currentModule = Module(name, location);
    }

    /// Ends the module that is being generated. A module must be active. Flushes the module to the module
    /// list and resets the current module.
    void endModule() {
        if (currentModule == std::nullopt) {
            SPDLOG_ERROR("Trying to end a module, but no module is active!");
            return;
        }
        // SPDLOG_TRACE("End module: {}", currentModule->name);
        modules.push_back(*currentModule);
        currentModule = std::nullopt;
    }

    /// Flushes the module if one is active, otherwise does nothing
    void maybeFlushModule() {
        if (currentModule != std::nullopt) {
            endModule();
        }
    }

    /// Executes the routine lambda if and only if a module is currently being worked on. The module is
    /// passed to the lambda.
    void doIfModuleIsActive(const std::function<void(Module &)> &routine) {
        if (currentModule != std::nullopt) {
            routine(*currentModule);
        }
    }

    std::optional<Module> getModuleByName(const std::string &name) const {
        for (const auto &module : modules) {
            if (module.name == name) {
                return module;
            }
        }
        return std::nullopt;
    }

    // void addPackage(const std::string &name) {
    //     packages.insert(name);
    // }

    // std::optional<Module> getPackageByName(const std::string &name) const {
    //     for (const auto &package : packages) {
    //         if (package == name) {
    //             return package;
    //         }
    //     }
    //     return std::nullopt;
    // }

    std::vector<Module> modules { };

    // FIXME make these be LangLocatable
    std::unordered_set<std::string> packages { };
    std::unordered_set<std::string> macros { };

private:
    std::optional<Module> currentModule = std::nullopt;
};

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Document, modules, packages, macros);

} // namespace slingshot::lang
