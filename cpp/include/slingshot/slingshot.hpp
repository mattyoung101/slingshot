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
#include <cstdint>

// This file is the bridge between Slingshot C++ and Slingshot Rust via rust-bindgen
// It uses simple C++ features like std::vector and std::string since it's easier than char**, but C-style
// function definitions, in the hope that rust-bindgen will support this.

/// Version of the Slingshot C++ code
#define SLINGSHOT_CPP_VERSION "0.0.1"

typedef struct {
    std::string message;
    uint32_t line;
} Diagnostic_t;

typedef struct {
    /// List of strings containing extracted completion tokens. 
    std::vector<std::string> tokens;
    /// List of diagnostics reported by Slang. 
    std::vector<Diagnostic_t> diagnostics;
} CompletionResult_t;

/**
 * Extracts completion tokens from the given input buffer. Also returns diagnostics if any exist.
 * @param document input text document (full text, not path)
 * @return completion result struct.
 */
CompletionResult_t slingshot_extract_completion_tokens(std::string document);

/**
 * @return the version of Slingshot C++ being used, corresponds to the SLINGSHOT_CPP_VERSION macro.
 */
std::string slingshot_get_cpp_version(void);

/**
 * @return the version of Slang being used
 */
std::string slingshot_get_slang_version(void);
