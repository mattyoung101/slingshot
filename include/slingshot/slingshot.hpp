// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "slingshot/compiler.hpp"
#include "slingshot/completion.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/remote_debug.hpp"
#include <cstddef>
#include <filesystem>
#include <fstream>
#include <lsp/messagehandler.h>
#include <memory>
#include <optional>
#include <slang/syntax/SyntaxTree.h>
#include <slang/text/SourceLocation.h>
#include <slang/text/SourceManager.h>
#include <spdlog/fmt/bundled/format.h>
#include <string>
#include <vector>

namespace slingshot {

extern bool g_running;
extern IndexManager g_indexManager;
extern CompilationManager g_compilerManager;
extern RemoteDebugger g_debugger;
extern std::shared_ptr<lsp::MessageHandler> g_msgHandler;
extern CompletionManager g_completionManager;

constexpr std::string CONFIG_MAJOR_VERSION = "1";

constexpr int REMOTE_DEBUGGER_PORT = 6942;

using SyntaxTreePtr = std::shared_ptr<slang::syntax::SyntaxTree>;

using SourceManagerPtr = std::shared_ptr<slang::SourceManager>;

/// Adds all the elements of B to the vector A
template <class T>
constexpr void addAll(std::vector<T> &a, const std::vector<T> &b) {
    a.insert(a.end(), b.begin(), b.end());
}

/// Returns true if the given vector 'v' contains the element 'x'
template <class T>
constexpr bool contains(const std::vector<T> &v, const T &x) {
    // https://stackoverflow.com/a/3450906/5007892
    return std::find(v.begin(), v.end(), x) != v.end();
}

constexpr std::string toString(const SourceLocation &loc, const SourceManagerPtr &srcMgr) {
    auto line = srcMgr->getLineNumber(loc);
    auto col = srcMgr->getColumnNumber(loc);
    return fmt::format("{}:{}", line, col);
}

constexpr std::string toString(const SourceRange &range, const SourceManagerPtr &srcMgr) {
    auto begin = toString(range.start(), srcMgr);
    auto end = toString(range.end(), srcMgr);

    return fmt::format("{}..{}", begin, end);
}

constexpr std::string toString(const std::optional<std::filesystem::path> &value) {
    if (value.has_value() && value != std::nullopt) {
        return value->string();
    }
    return "(null)";
}

constexpr std::string readFile(const std::filesystem::path &path) {
    // TODO does this bugger all error checking
    std::ifstream t(path);
    std::stringstream buffer;
    buffer << t.rdbuf();
    return buffer.str();
}

constexpr size_t timeNowNs() {
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now().time_since_epoch())
        .count();
}

// https://stackoverflow.com/a/217605

// Trim from the start (in place)
inline void ltrim(std::string &s) {
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) { return !std::isspace(ch); }));
}

// Trim from the end (in place)
inline void rtrim(std::string &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) { return !std::isspace(ch); }).base(),
        s.end());
}

// Trim from both ends (in place)
inline void trim(std::string &s) {
    rtrim(s);
    ltrim(s);
}

// https://stackoverflow.com/a/3418285
inline bool replace(std::string &str, const std::string &from, const std::string &to) {
    size_t start_pos = str.find(from);
    if (start_pos == std::string::npos)
        return false;
    str.replace(start_pos, from.length(), to);
    return true;
}

} // namespace slingshot
