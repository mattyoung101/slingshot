// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "slang/diagnostics/DiagnosticClient.h"
#include "slang/diagnostics/DiagnosticEngine.h"
#include "slang/diagnostics/Diagnostics.h"
#include "slang/text/SourceManager.h"
#include <filesystem>
#include <lsp/types.h>
#include <memory>
#include <optional>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

namespace slingshot {

using namespace slang;

/// Types of completions we can recommend to the user
enum class CompletionType {
    /// Do not recommend anything
    None,

    /// A module
    Module,

    /// A port from the module the cursor is in
    PortSameModule,

    /// A port from the module we are instantiating
    PortInstantiatedModule,

    /// A "variable" from the module that is being instantiated
    VariableSameModule,

    /// A `defined` macro
    Macro,

    /// Either posedge or nedge
    Edge,

    /// The keywords: logic, wire, reg
    Logic,

    /// An always_ff, always_comb or always_latch snippet
    Always,

    /// An SV system task, e.g. $display, $error, etc
    SystemTask,
};

const std::vector<std::string> SYSTEM_TASKS = {
    "display",
    "monitor",
    "write",
    "strobe",
    "error",
    "fatal",
    "info",
    "warning",
    "clog2",
    "finish",
    "stop",
    "fopen",
    "fscanf",
    "fwrite",
    "fgets",
    "readmemb",
    "readmemh",
    "write",
    "floor",
    "ceil",
    "countones",
    "countbits",
    "time",
    "signed",
    "unsigned",

    // These pollute the completion list quite a bit, and I can't imagine they are seriously commonly
    // used, so I'm not including them for now.
    // "asin",
    // "acos",
    // "atan",
    // "atan2",
    // "sin",
    // "cos",
    // "tan",
    // "ln",
    // "log10",
    // "exp",
    // "sqrt",
    // "hypot",
    // "pow",
};

class CompletionManager {
public:
private:
};

} // namespace slingshot
