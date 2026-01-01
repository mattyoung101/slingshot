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
#include <lsp/json/json.h>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <memory>
#include <nlohmann/json_fwd.hpp>
#include <optional>
#include <slang/syntax/SyntaxTree.h>
#include <spdlog/spdlog.h>
#include <sstream>
#include <string>

using namespace slingshot;

void IndexManager::insert(const std::filesystem::path &path, const std::string &document, bool isIndex) {
    // ensure absolute
    auto realPath = std::filesystem::absolute(path);
    SPDLOG_TRACE("Insert {}", realPath.string());

    auto hash = ankerl::unordered_dense::detail::wyhash::hash(document.c_str(), document.size());

    auto maybeEntry = retrieve(realPath);

    // take the mutex before we push to the index
    auto guard = acquireWriteLock();
    if (maybeEntry == std::nullopt) {
        SPDLOG_DEBUG("Path {} not yet in index, inserting brand new entry", realPath.string());
        index[realPath] = std::make_shared<IndexEntry>(realPath, hash);
        // make it available in the document graph as well
        documentGraph->insertDocument(realPath);
    } else {
        SPDLOG_TRACE("Path {} already in index, invalidating and updating", realPath.string());
        index[realPath]->invalidate(hash);
    }

    // regardless, schedule a compilation job for this
    if (isIndex) {
        g_compilerManager.indexDocument(document, realPath);
    } else {
        g_compilerManager.submitCompilationJob(document, realPath);
    }
}

void IndexManager::insert(const std::filesystem::path &path, bool isIndex) {
    // read the file to a string
    // TODO does this bugger all error checking
    std::ifstream t(path);
    std::stringstream buffer;
    buffer << t.rdbuf();

    insert(path, buffer.str(), isIndex);
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
        insert(std::filesystem::absolute(path), true);
        return;
    }

    isStillQueueingIndexJobs = true;

    // first, we need to tell the server about our token
    lsp::requests::Window_WorkDoneProgress_Create::Params create("SlingshotIndexProgress");
    auto result = g_msgHandler->sendRequest<lsp::requests::Window_WorkDoneProgress_Create>(std::move(create));

    // NOW, we can actually initiate the work done progress, in a really really stupid way
    // reference:
    // https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initiatingWorkDoneProgress
    lsp::notifications::Progress::Params beginMsg;
    beginMsg.token = "SlingshotIndexProgress";
    beginMsg.value = lsp::toJson(lsp::WorkDoneProgressBegin());
    g_msgHandler->sendNotification<lsp::notifications::Progress>(std::move(beginMsg));

    isInitialIndexInProgress = true;

    for (const auto &dirEntry : std::filesystem::recursive_directory_iterator(path)) {
        // make sure the extension is in (sv, v, svh, vh)
        auto ext = dirEntry.path().extension().string();
        if (ext != ".sv" && ext != ".v" && ext != ".svh" && ext != ".vh") {
            continue;
        }
        SPDLOG_INFO("Discovered document: {}", dirEntry.path().string());
        insert(dirEntry, true);
    }

    // we've finished queueing jobs now, so later at some point we can officially terminate the indexing
    isStillQueueingIndexJobs = false;
}

ankerl::unordered_dense::set<std::shared_ptr<slang::syntax::SyntaxTree>> IndexManager::getAllSyntaxTrees() {
    auto lock = acquireReadLock();
    ankerl::unordered_dense::set<std::shared_ptr<slang::syntax::SyntaxTree>> out;
    SPDLOG_DEBUG("Attempting to find all syntax trees");

    for (const auto &entry : index) {
        const auto &[path, indexEntry] = entry;

        // if (indexEntry->valid && indexEntry->tree != nullptr) {
        if (indexEntry->tree != nullptr) {
            SPDLOG_TRACE("Add syntax tree: {}", path.string());
            out.insert(indexEntry->tree);
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

std::string IndexManager::dumpSources() {
    auto lock = acquireReadLock();
    std::stringstream stream;
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        if (value->tree != nullptr) {
            stream << fmt::format(
                "=== Document: {} ===\n{}\n\n", key.string(), value->tree->root().toString());
        }
    }
    return stream.str();
}

std::optional<std::shared_ptr<SyntaxTree>> IndexManager::locateDocumentForModule(const std::string &name) {
    auto lock = acquireReadLock();
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        if (value->tree != nullptr && value->doc != std::nullopt) {
            // we have a valid document, let's investigate
            auto query = value->doc->getModuleByName(name);
            if (query.has_value() && query != std::nullopt) {
                SPDLOG_DEBUG("Found document for module '{}': {}", name, key.string());
                return value->tree;
            }
        }
    }
    return std::nullopt;
}

std::optional<std::shared_ptr<SyntaxTree>> IndexManager::locateDocumentForPackage(const std::string &name) {
    auto lock = acquireReadLock();
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        if (value->tree != nullptr && value->doc != std::nullopt) {
            // we have a valid document, let's investigate
            auto query = value->doc->getPackageByName(name);
            if (query.has_value() && query != std::nullopt) {
                SPDLOG_DEBUG("Found document for package '{}': {}", name, key.string());
                return value->tree;
            }
        }
    }
    return std::nullopt;
}
