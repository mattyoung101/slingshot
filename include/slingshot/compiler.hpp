// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "BS_thread_pool.hpp"
#include "ankerl/unordered_dense.h"
#include "slang/diagnostics/AllDiags.h"
#include "slang/diagnostics/Diagnostics.h"
#include "slang/text/SourceManager.h"
#include <filesystem>
#include <mutex>
#include <optional>
#include <slang/text/SourceLocation.h>
#include <string>

namespace slingshot {

using namespace slang;

class CompilationManager {
public:
    /// Submits a compilation job asynchronously
    void submitCompilationJob(const std::string &document, const std::filesystem::path &path);

    std::optional<Diagnostics> getDiagnostics(const std::filesystem::path &path);

private:
    BS::thread_pool<> pool;
    ankerl::unordered_dense::map<std::filesystem::path, Diagnostics> diags;
    ankerl::unordered_dense::map<std::filesystem::path, SourceBuffer> slangBufs;
    SourceManager sourceMgr;
    std::mutex mutex;

};

} // namespace slingshot
