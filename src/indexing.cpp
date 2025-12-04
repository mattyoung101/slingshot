// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/indexing.hpp"
#include "slingshot/slingshot.hpp"
#include "slingshot/compiler.hpp"
#include <ankerl/unordered_dense.h>
#include <optional>
#include <spdlog/spdlog.h>

using json = nlohmann::json;
using namespace slingshot;

void IndexManager::insert(const std::filesystem::path &path, const std::string &document) {
    SPDLOG_DEBUG("IndexManager::insert {}", path.string());

    auto hash = ankerl::unordered_dense::detail::wyhash::hash(document.c_str(), document.size());
    IndexEntry entry { .version = INDEX_VERSION, .path = path, .hash = hash };

    if (retrieve(path, hash) == std::nullopt) {
        SPDLOG_DEBUG("Not yet in index, so inserting it");
        index[path] = entry;

        // and also schedule a compilation job for this
        g_compilerManager.submitCompilationJob(document, path);
    } else {
        SPDLOG_DEBUG("Already in index with this path and hash, not inserted");
    }
}

std::optional<IndexEntry> IndexManager::retrieve(const std::filesystem::path &path, uint64_t hash) const {
    if (!index.contains(path)) {
        return std::nullopt;
    }

    auto entry = index.at(path);
    if (entry.hash != hash) {
        return std::nullopt;
    }

    return entry;
}

std::optional<IndexEntry> IndexManager::retrieve(const std::filesystem::path &path) const {
    if (!index.contains(path)) {
        return std::nullopt;
    }
    return index.at(path);
}
