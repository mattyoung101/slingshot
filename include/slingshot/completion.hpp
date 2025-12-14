// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "slingshot/indexing.hpp"
#include "slingshot/language.hpp"
#include <filesystem>
#include <lsp/types.h>
#include <slang/syntax/AllSyntax.h>
#include <slang/syntax/SyntaxVisitor.h>
#include <slang/text/SourceLocation.h>
#include <slang/text/SourceManager.h>
#include <spdlog/spdlog.h>
#include <string>
#include <utility>
#include <vector>

namespace slingshot {

using namespace slang;
using namespace slang::syntax;

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
    CompletionSyntaxVisitor(const SourceLocation &cursor, lang::Document document)
        : cursor(cursor)
        , doc(std::move(document)) {
    }

    void handle(const EventControlWithExpressionSyntax &syntax);

    void handle(const ExpressionSyntax &syntax);

    void handle(const AnsiPortListSyntax &syntax);

    void handle(const ModuleDeclarationSyntax &syntax);

    /// The recommended things to complete
    std::vector<lsp::CompletionItem> recommendations;

    /// The name of the current module the cursor is in, if it is in one
    std::optional<std::string> activeModule;

private:
    /// Cursor position
    SourceLocation cursor;

    /// Lifted document
    lang::Document doc;

    void recommend(const std::vector<lsp::CompletionItem> &completions, const std::string &what);
};

class CompletionManager {
public:
    /// Generates completions for the given document at the given path
    static std::vector<lsp::CompletionItem> getCompletions(
        const std::filesystem::path &path, const lsp::Position &pos, const IndexEntry::Ptr &indexEntry);

private:
};

/// Utility only class for generating syntax completions
class CompletionGenerator {
public:
    /// Generates completion items for CompletionType::Logic
    static std::vector<lsp::CompletionItem> generateLogic();

    /// Generates completion items for posedge and negedge
    static std::vector<lsp::CompletionItem> generateEdge();

    /// Generates always_ff, always_comb, always_latch blocks
    static std::vector<lsp::CompletionItem> generateAlways();

    static std::vector<lsp::CompletionItem> generateSystemTasks();

    static std::vector<lsp::CompletionItem> generateInputOutput();

    static std::vector<lsp::CompletionItem> generateVariableSameModule(
        const std::optional<std::string> &activeModule, const lang::Document &doc);
};

} // namespace slingshot
