/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#pragma once
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

// This file is the bridge between Slingshot C++ and Slingshot Rust via rust-bindgen.
// This was originally going to be C++, but since we're not using the cxx crate (only bindgen), we are forced
// to use pure C, since apparently C++ doesn't have a stable ABI, or something. That's fine, just annoying
// and more work. 
//
// The Rust side will need to be checked carefully to make sure it's not leaking memory. Leaks in an LSP
// like this would be pretty catastrophic.

/// Version of the Slingshot C++ code
#define SLINGSHOT_CPP_VERSION "0.1.0"

typedef struct {
    /// Diagnostic message
    char *message;
    /// Line number diagnostic occurs on
    size_t line;
    /// Position in line where diagnostic occurs on
    size_t pos;
} Diagnostic_t;

typedef struct {
    /// List of strings containing extracted completion tokens. 
    char **tokens;
    /// Number of strings in the tokens array
    size_t numTokens;
    /// List of diagnostics reported by Slang. 
    Diagnostic_t *diagnostics;
    /// Number of diagnostics in the diagnostics array
    size_t numDiagnostics;
} CompletionResult_t;

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Extracts completion tokens from the given input buffer. Also returns diagnostics if any exist.
 * @param document input text document (full text, not path)
 * @param debug if true, run in debug mode (e.g. print logs, etc), otherwise run in normal mode
 * @return completion result struct.
 */
CompletionResult_t slingshot_extract_completion_tokens(const char *document, bool debug);

/**
 * Frees the memory allocated by a CompletionResult_t struct
 * @param result struct to be freed
 */
void slingshot_free_completion(CompletionResult_t result);

/**
 * Simple wrapper around free() for calling from Rust
 * @param str string to be freed
 */
void slingshot_free_str(char *str);

/**
 * @return the version of Slingshot C++ being used, corresponds to the SLINGSHOT_CPP_VERSION macro. heap
 * allocated, must be freed.
 */
char *slingshot_get_cpp_version(void);

/**
 * @return the version of Slang being used, heap allocated, must be freed
 */
char *slingshot_get_slang_version(void);

#ifdef __cplusplus
};
#endif

