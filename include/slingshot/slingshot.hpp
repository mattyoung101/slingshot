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

namespace slingshot {

extern bool g_running;
extern IndexManager g_indexManager;
extern CompilationManager g_compilerManager;

constexpr std::string CONFIG_VERSION = "1.0.0";

} // namespace slingshot
