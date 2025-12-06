// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once

// Parts of this are based on slang-server ServerDiagClient.cpp, which is available under the MIT licence:
//
// Copyright (c) 2024-2025 Hudson River Trading LLC <opensource@hudson-trading.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#include <lsp/types.h>
#include <lsp/uri.h>
#include <optional>
#include <slang/text/SourceLocation.h>
#include <slang/text/SourceManager.h>
#include <span>
#include <spdlog/spdlog.h>
#include <string_view>

namespace slingshot {

using namespace slang;

// Parts of this are based on slang-server Converters.cpp, which is available under the MIT licence:
//
// Copyright (c) 2024-2025 Hudson River Trading LLC <opensource@hudson-trading.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

inline lsp::Position toPosition(
    const SourceLocation &loc, const std::shared_ptr<SourceManager> &sourceManager) {
    auto character = sourceManager->getColumnNumber(loc);
    return lsp::Position { .line = static_cast<lsp::uint>(sourceManager->getLineNumber(loc) - 1),
        .character = static_cast<lsp::uint>(character > 0 ? character - 1 : 0) };
}

inline lsp::Range toRange(const SourceRange &range, const std::shared_ptr<SourceManager> &sourceManager) {
    return lsp::Range { .start = toPosition(range.start(), sourceManager),
        .end = toPosition(range.end(), sourceManager) };
}

inline lsp::Range toOriginalRange(
    const SourceRange &range, const std::shared_ptr<SourceManager> &sourceManager) {
    auto origRange = sourceManager->getFullyOriginalRange(range);
    return toRange(origRange, sourceManager);
}

inline lsp::Range toRange(
    const SourceLocation &loc, const SourceManager &sourceManager, const size_t length) {

    auto character = sourceManager.getColumnNumber(loc);
    lsp::Position start { .line = static_cast<lsp::uint>(sourceManager.getLineNumber(loc) - 1),
        .character = static_cast<lsp::uint>(character > 0 ? character - 1 : 0) };
    lsp::Position end { start };
    end.character += length;
    return lsp::Range { .start = start, .end = end };
}

inline lsp::Location toLocation(
    const SourceRange &range, const std::shared_ptr<SourceManager> &sourceManager) {
    // TODO: make this logic just one function in source manager
    auto declRange = range;

    // Get location of `MACRO_USAGE if from a macro expansion

    // FIXME(mlyoung): getMacroExpansions seems to be patched in by slang-server people, or at least it's not
    // in my build of slang!

    // auto locs = sourceManager.getMacroExpansions(range.start());
    // if (locs.size()) {
    //     auto macroInfo = sourceManager.getMacroInfo(locs.back());
    //     declRange = macroInfo ? macroInfo->expansionRange : sourceManager.getFullyOriginalRange(range);
    // }

    auto uri = lsp::Uri::parse(
        std::string("file://") + sourceManager->getFullPath(declRange.start().buffer()).string());
    SPDLOG_DEBUG("URI is: {}", uri.toString());

    return lsp::Location { .uri = uri, .range = toRange(declRange, sourceManager) };
}

inline lsp::Location toLocation(
    const SourceLocation &loc, const std::shared_ptr<SourceManager> &sourceManager) {
    auto uri = lsp::Uri::parse(std::string("file://") + sourceManager->getFullPath(loc.buffer()).string());
    SPDLOG_DEBUG("URI is: {}", uri.toString());

    return lsp::Location { .uri = uri,
        .range
        = lsp::Range { .start = toPosition(loc, sourceManager), .end = toPosition(loc + 1, sourceManager) } };
}

inline std::optional<lsp::Location> getLocation(SourceLocation loc, std::span<const SourceRange> ranges,
    std::string_view message, const std::shared_ptr<SourceManager> &sourceMgr) {
    bool hasLocation = loc.buffer() != SourceLocation::NoLocation.buffer();
    if (ranges.empty()) {
        if (hasLocation) {
            return toLocation(loc, sourceMgr);
        }
        SPDLOG_ERROR("Diagnostic has no ranges and no location: {}", message);

    } else {
        // collapse ranges into one, if they're all in the same buffer
        SourceRange totalRange = ranges[0];
        for (const auto &range : ranges.subspan(1)) {
            if (range.start().buffer() != totalRange.start().buffer()) {
                SPDLOG_ERROR("Diagnostic has ranges in multiple buffers: {}", message);
            } else {
                totalRange.start() = std::min(totalRange.start(), range.start());
                totalRange.end() = std::max(totalRange.end(), range.end());
            }
        }
        if (hasLocation) {
            if (loc.buffer() != totalRange.start().buffer()) {
                SPDLOG_ERROR("Diagnostic location and ranges are in different buffers: {}", message);
            } else {
                totalRange.start() = std::min(totalRange.start(), loc);
                totalRange.end() = std::max(totalRange.end(), loc);
            }
        }
        return toLocation(totalRange, sourceMgr);
    }
    return std::nullopt;
}

}; // namespace slingshot
