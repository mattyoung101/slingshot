// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/indexing.hpp"
#include "slingshot/compiler.hpp"
#include "slingshot/lang_lifter.hpp"
#include "slingshot/language.hpp" // NECESSARY for JSON conversion
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <fstream>
#include <lsp/types.h>
#include <memory>
#include <nlohmann/json_fwd.hpp>
#include <optional>
#include <slang/syntax/SyntaxTree.h>
#include <spdlog/spdlog.h>
#include <sstream>
#include <string>

using namespace slingshot;

void IndexManager::insert(const std::filesystem::path &path, const std::string &document) {
    // ensure absolute
    auto realPath = std::filesystem::absolute(path);
    SPDLOG_TRACE("Insert {}", realPath.string());

    auto hash = ankerl::unordered_dense::detail::wyhash::hash(document.c_str(), document.size());

    auto maybeEntry = retrieve(realPath);

    // take the mutex before we push to the index
    auto guard = acquireWriteLock();
    if (maybeEntry == std::nullopt) {
        SPDLOG_DEBUG("realPath {} not yet in index, inserting brand new entry", realPath.string());
        index[realPath] = std::make_shared<IndexEntry>(realPath, hash);
    } else {
        SPDLOG_TRACE("realPath {} already in index, invalidating and updating", realPath.string());
        index[realPath]->invalidate(hash);
    }

    // regardless, schedule a compilation job for this
    g_compilerManager.submitCompilationJob(document, realPath);
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
    SPDLOG_TRACE("Now associating parse");
    auto result = retrieve(path);

    // hold a lock guard, since we're calling this from CompilerManager which is multi-threaded
    auto lock = acquireWriteLock();
    if (result.has_value()) {
        (*result)->tree = tree;
        SPDLOG_TRACE("Result has value, attempting to mark as valid");
        (*result)->makeValid();
    } else {
        SPDLOG_WARN("Path {} somehow not in the index!", path.string());
    }
}

void IndexManager::associateLangDoc(const std::filesystem::path &path, const lang::Document &doc) {
    auto result = retrieve(path);

    // hold a lock guard, since we're calling this from CompilerManager which is multi-threaded
    auto lock = acquireWriteLock();
    if (result.has_value()) {
        (*result)->doc = doc;
    } else {
        SPDLOG_WARN("Path {} somehow not in the index!", path.string());
    }
}

std::optional<IndexEntry::Ptr> IndexManager::retrieve(const std::filesystem::path &path, uint64_t hash) {
    auto guard = acquireReadLock();
    auto realPath = std::filesystem::absolute(path);
    if (!index.contains(realPath)) {
        return std::nullopt;
    }

    auto entry = index.at(realPath);
    if (entry->hash != hash) {
        return std::nullopt;
    }

    return entry;
}

std::optional<IndexEntry::Ptr> IndexManager::retrieve(const std::filesystem::path &path) {
    auto guard = acquireReadLock();
    auto realPath = std::filesystem::absolute(path);
    if (!index.contains(realPath)) {
        return std::nullopt;
    }
    return index.at(realPath);
}

void IndexManager::walkDir(const std::filesystem::path &path) {
    SPDLOG_INFO("Walk dir: {}", path.string());

    if (!std::filesystem::is_directory(path)) {
        // we lie a bit here, submit directly for indexing if they told us its a path but it's actually a
        // single file
        SPDLOG_INFO("Discovered (direct) document: {}", path.string());
        insert(std::filesystem::absolute(path));
        return;
    }

    for (const auto &dirEntry : std::filesystem::recursive_directory_iterator(path)) {
        SPDLOG_INFO("Discovered document: {}", dirEntry.path().string());
        insert(std::filesystem::absolute(dirEntry));
    }
}

std::vector<std::shared_ptr<slang::syntax::SyntaxTree>> IndexManager::getAllSyntaxTrees() {
    auto lock = acquireReadLock();
    std::vector<std::shared_ptr<slang::syntax::SyntaxTree>> out;
    SPDLOG_DEBUG("Attempting to find all syntax trees");

    for (const auto &entry : index) {
        const auto &[path, indexEntry] = entry;
        SPDLOG_DEBUG("Visiting path {} to see if it has a syntax tree", path.string());

        // if (indexEntry->valid && indexEntry->tree != nullptr) {
        if (indexEntry->tree != nullptr) {
            out.push_back(indexEntry->tree);
        }
    }

    return out;
}

std::string IndexManager::debugDump() {
    auto lock = acquireReadLock();
    std::stringstream stream;
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        stream << fmt::format("{}    0x{:X}\n", key.string(), value->hash);
    }
    return stream.str();
}

std::string IndexManager::dumpLangTrees() {
    auto lock = acquireReadLock();
    std::stringstream stream;
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        if (value->doc != std::nullopt) {
            auto doc = *value->doc;
            nlohmann::json docJson = doc;
            stream << fmt::format("Document: {}\n{}\n\n", key.string(), docJson.dump(4));
        }
    }
    return stream.str();
}
