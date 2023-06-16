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
#include "slingshot/slingshot.hpp"

namespace slingshot {

/// The CompletionTokenExtractor uses Slang to extract tokens from the input source that may be useful for
/// auto-complete.
class CompletionTokenExtractor {
public:
    CompletionTokenExtractor() = default;
    ~CompletionTokenExtractor() = default;

    // TODO: consider scopes
    
    /// Finds completion tokens in the given document.
    /// @param document the full contents of the document
    /// @return unsorted list of tokens that could be used for completion in the document
    CompletionResult extractTokens(std::string document);
};

};
