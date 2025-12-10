// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "ankerl/unordered_dense.h"
#include "slingshot/indexing.hpp"
#include "slingshot/compiler.hpp"
#include "slingshot/remote_debug.hpp"
#include "slingshot/completion.hpp"
#include <lsp/messagehandler.h>
#include <memory>
#include <slang/syntax/SyntaxTree.h>
#include <vector>

namespace slingshot {

extern bool g_running;
extern IndexManager g_indexManager;
extern CompilationManager g_compilerManager;
extern RemoteDebugger g_debugger;
extern std::shared_ptr<lsp::MessageHandler> g_msgHandler;
extern CompletionManager g_completionManager;

constexpr std::string CONFIG_VERSION = "1.0.0";

constexpr int REMOTE_DEBUGGER_PORT = 6942;

using SyntaxTreePtr = std::shared_ptr<slang::syntax::SyntaxTree>;

/// Adds all the elements of B to the vector A
template <class T>
inline void addAll(std::vector<T> &a, const std::vector<T> &b) {
    a.insert(a.end(), b.begin(), b.end());
}

} // namespace slingshot
