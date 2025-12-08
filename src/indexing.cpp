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
#include <lsp/types.h>
#include <memory>
#include <optional>
#include <slang/syntax/SyntaxTree.h>
#include <spdlog/spdlog.h>
#include <sstream>
#include <string>

using json = nlohmann::json;
using namespace slingshot;

void IndexManager::insert(const std::filesystem::path &path, const std::string &document) {
    SPDLOG_DEBUG("IndexManager::insert {}", path.string());

    auto hash = ankerl::unordered_dense::detail::wyhash::hash(document.c_str(), document.size());
    IndexEntry entry { .version = INDEX_VERSION,
        .path = path,
        .hash = hash,
        .tree = nullptr,
        .diagnostics = std::vector<lsp::Diagnostic>() };

    if (retrieve(path, hash) == std::nullopt) {
        // take the mutex before we push to the index
        {
            std::lock_guard<std::mutex> guard(g_indexManager.lock);

            SPDLOG_DEBUG("Not yet in index, so inserting it");
            index[path] = std::make_shared<IndexEntry>(entry);
        }

        // and also schedule a compilation job for this
        g_compilerManager.submitCompilationJob(document, path);
    } else {
        SPDLOG_DEBUG("Already in index with this path and hash, not inserted");
    }
}

void IndexManager::insert(const std::filesystem::path &path) {
    // read the file to a string
    // TODO does this bugger all error checking
    std::ifstream t(path);
    std::stringstream buffer;
    buffer << t.rdbuf();

    insert(path, buffer.str());
}

void IndexManager::associateParse(
    const std::filesystem::path &path, const std::shared_ptr<slang::syntax::SyntaxTree> &tree) {
    // hold a lock guard, since we're calling this from CompilerManager which is multi-threaded
    std::lock_guard<std::mutex> guard(g_indexManager.lock);

    auto result = retrieve(path);
    if (result.has_value()) {
        (*result)->tree = tree;
    } else {
        SPDLOG_WARN("Path {} somehow not in the index!", path.string());
    }
}

void IndexManager::associateDiagnostics(
    const std::filesystem::path &path, const std::vector<lsp::Diagnostic> &diagnostics) {
    // hold a lock guard, since we're calling this from CompilerManager which is multi-threaded
    std::lock_guard<std::mutex> guard(g_indexManager.lock);

    auto result = retrieve(path);
    if (result.has_value()) {
        (*result)->diagnostics = diagnostics;
    } else {
        SPDLOG_WARN("Path {} somehow not in the index!", path.string());
    }
}

// NOTE DOES NOT LOCK
std::optional<IndexEntry::Ptr> IndexManager::retrieve(
    const std::filesystem::path &path, uint64_t hash) const {
    if (!index.contains(path)) {
        return std::nullopt;
    }

    auto entry = index.at(path);
    if (entry->hash != hash) {
        return std::nullopt;
    }

    return entry;
}

// NOTE DOES NOT LOCK
std::optional<IndexEntry::Ptr> IndexManager::retrieve(const std::filesystem::path &path) const {
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
        SPDLOG_INFO("Discovered (direct) document: {}", path.string());
        insert(path);
        return;
    }

    for (const auto &dirEntry : std::filesystem::recursive_directory_iterator(path)) {
        SPDLOG_INFO("Discovered document: {}", dirEntry.path().string());
        insert(dirEntry);
    }
}

std::string IndexManager::debugDump() {
    std::stringstream stream;
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        stream << fmt::format(
            "{}    0x{:X}    {} diags\n", key.string(), value->hash, value->diagnostics.size());
    }
    return stream.str();
}
