// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/indexing.hpp"
#include "slingshot/compiler.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <fstream>
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

void IndexManager::insert(const std::filesystem::path &path) {
    // read the file to a string
    std::ifstream t(path);
    std::stringstream buffer;
    buffer << t.rdbuf();

    insert(path, buffer.str());
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

void IndexManager::walkDir(const std::filesystem::path &path) {
    SPDLOG_INFO("Walk dir: {}", path.string());

    if (!std::filesystem::is_directory(path)) {
        // we lie a bit here, submit directly for indexing if they told us its a path but it's actually a
        // single file
        insert(path);
        return;
    }

    for (const auto &dirEntry : std::filesystem::recursive_directory_iterator(path)) {
        if (dirEntry.is_directory()) {
            // recursive!
            walkDir(dirEntry);
        } else {
            // it's a file, so we can just index it straight away
            insert(dirEntry);
        }
    }
}
