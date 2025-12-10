// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "slingshot/indexing.hpp"
#include <filesystem>
#include <lsp/types.h>
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <slang/text/SourceManager.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

namespace slingshot {

using namespace slang;
using namespace slang::syntax;

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

/// A syntax visitor that walks the syntax tree, taking into account the cursor position
class CompletionSyntaxVisitor : public SyntaxVisitor<CompletionSyntaxVisitor> {
public:
    CompletionSyntaxVisitor(const SourceLocation &cursor)
        : cursor(cursor) {
    }

    void handle(const EventControlWithExpressionSyntax &syntax);

    void handle(const ExpressionSyntax &syntax);

    /// The recommended things to complete
    std::vector<CompletionType> recommendations;

private:
    /// Cursor position
    SourceLocation cursor;
};

class CompletionManager {
public:
    /// Generates completions for the given document at the given path
    static std::vector<lsp::CompletionItem> getCompletions(
        const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry);

private:
};

class CompletionGenerator {
public:
    /// Processes each type of completion specified, and generates the correct LSP completion item(s) for it
    static std::vector<lsp::CompletionItem> transformAll(const std::vector<CompletionType> &completions);

    /// Generates completion items for CompletionType::Logic
    static std::vector<lsp::CompletionItem> generateLogic();

    /// Generates completion items for posedge and negedge
    static std::vector<lsp::CompletionItem> generateEdge();
};

} // namespace slingshot
