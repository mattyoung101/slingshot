/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#pragma once
#include <vector>
#include <string>

/// The CompilationManager manages compiling SystemVerilog sources with Slang and returning the resulting
/// slang::Compilation object. It also collects diagnostics for later extraction.
class CompilationManager {
public:
    CompilationManager() = default;
    ~CompilationManager() = default;
};
