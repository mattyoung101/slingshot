// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/compiler.hpp"
#include "slingshot/indexing.hpp"
#include <ankerl/unordered_dense.h>
#include <mutex>
#include <spdlog/spdlog.h>

using namespace slingshot;

void CompilationManager::submitCompilationJob(const std::string &document, const std::filesystem::path &path) {
    SPDLOG_DEBUG("Submitting document");

    {
        std::lock_guard<std::mutex> guard(mutex);
        auto buf = sourceMgr.assignText(document);
        slangBufs[path] = buf;
    }
}
