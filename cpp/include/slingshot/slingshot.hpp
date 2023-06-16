/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#pragma once
#include <string>
#include <vector>
#include <cstdint>

/// Version of the Slingshot C++ code
#define SLINGSHOT_CPP_VERSION "0.0.1"

struct CompletionResult {
    /// List of lists of completion tokens. Each list in the outer list is one context window deeper than
    /// the previous list (TODO better explanation)
    std::vector<std::vector<std::string>> tokens;
    /// Index of current cursor position in the token list
    uint32_t cursorPos;
};

struct SlingshotInvocation {
    std::string path;
};
