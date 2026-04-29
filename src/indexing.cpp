// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/indexing.hpp"
#include "slingshot/compiler.hpp"
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
#include <vector>

using namespace slingshot;

void IndexManager::insert(const std::filesystem::path &path, const std::string &document, bool isIndex) {
    // ensure absolute
    auto realPath = std::filesystem::absolute(path);
    SPDLOG_TRACE("Insert {}", realPath.string());

    auto hash = ankerl::unordered_dense::detail::wyhash::hash(document.c_str(), document.size());
    auto maybeEntry = retrieve(realPath);

    // take the mutex before we push to the index
    auto lock = acquireLock();
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
    lock.unlock();
    g_compilerManager.submitCompilationJob(document, realPath, isIndex);
}

void IndexManager::insert(const std::filesystem::path &path, bool isIndex) {
    // read the file to a string
    // TODO does this bugger all error checking
    const std::ifstream t(path);
    std::stringstream buffer;
    buffer << t.rdbuf();

    insert(path, buffer.str(), isIndex);
}

void IndexManager::associateParse(
    const std::filesystem::path &path, const std::shared_ptr<slang::syntax::SyntaxTree> &tree) {
    SPDLOG_TRACE("Now associating parse");
    // since we have a recursive lock, we'll hold it across the entire duration of this routine
    auto lock = acquireLock();
    auto result = retrieve(path);

    if (result.has_value()) {
        (*result)->tree = tree;
        // we'll also update the last updated time, since this is valid
        (*result)->lastUpdated = timeNowNs();
        SPDLOG_TRACE("Result has value, attempting to mark as valid");
        (*result)->makeValid();
    } else {
        SPDLOG_WARN("Path {} somehow not in the index!", path.string());
    }
}

void IndexManager::associateLangDoc(const std::filesystem::path &path, const lang::Document &doc) {
    // since we have a recursive lock, we'll hold it across the entire duration of this routine
    auto lock = acquireLock();
    auto result = retrieve(path);

    if (result.has_value()) {
        (*result)->doc = doc;
    } else {
        SPDLOG_WARN("Path {} somehow not in the index!", path.string());
    }
}

std::optional<IndexEntry::Ptr> IndexManager::retrieve(const std::filesystem::path &path, uint64_t hash) {
    auto guard = acquireLock();
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
    auto guard = acquireLock();
    auto realPath = std::filesystem::absolute(path);
    if (!index.contains(realPath)) {
        return std::nullopt;
    }
    return index.at(realPath);
}

void IndexManager::beginInitialIndexing() {
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

    for (const auto &dirEntry : std::filesystem::recursive_directory_iterator(path)) {
        // make sure the extension is in (sv, v, svh, vh)
        auto ext = dirEntry.path().extension().string();
        if (ext != ".sv" && ext != ".v" && ext != ".svh" && ext != ".vh") {
            continue;
        }
        SPDLOG_INFO("Discovered document: {}", dirEntry.path().string());
        insert(dirEntry, true);
    }
}

ankerl::unordered_dense::set<std::shared_ptr<slang::syntax::SyntaxTree>> IndexManager::getAllSyntaxTrees() {
    auto lock = acquireLock();
    ankerl::unordered_dense::set<std::shared_ptr<slang::syntax::SyntaxTree>> out;
    SPDLOG_DEBUG("Attempting to find all syntax trees");

    for (const auto &entry : index) {
        const auto &[path, indexEntry] = entry;

        if (indexEntry->tree != nullptr) {
            SPDLOG_TRACE("Add syntax tree: {}", path.string());
            out.insert(indexEntry->tree);
        }
    }

    return out;
}

std::string IndexManager::debugDump() {
    auto lock = acquireLock();
    std::stringstream stream;
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        stream << fmt::format("{}    0x{:X}\n", key.string(), value->hash);
    }
    return stream.str();
}

std::string IndexManager::dumpLangTrees() {
    auto lock = acquireLock();
    std::stringstream stream;
    for (const auto &entry : index) {
        const auto &[key, value] = entry;
        if (value->doc != std::nullopt) {
            auto doc = *value->doc;
            const nlohmann::json docJson = doc;
            stream << fmt::format("Document: {}\n{}\n\n", key.string(), docJson.dump(4));
        }
    }
    return stream.str();
}

std::string IndexManager::dumpSources() {
    auto lock = acquireLock();
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

std::vector<lang::Document> IndexManager::getAllLangDocs() {
    auto lock = acquireLock();
    std::vector<lang::Document> out;
    for (const auto &[key, value] : index) {
        if (value->doc.has_value()) {
            out.push_back(*value->doc);
        }
    }
    return out;
}

void IndexManager::parseFListFile(const std::filesystem::path &path) {
    SPDLOG_DEBUG("Parse F-list file: {}", path.string());
    if (!std::filesystem::exists(path)) {
        SPDLOG_ERROR("F-list file '{}' does not exist", path.string());
        return;
    }

    auto contents = readFile(path);

    // https://stackoverflow.com/a/12514641
    std::istringstream iss(contents);
    for (std::string line; std::getline(iss, line);) {
        trim(line);
        if (line.empty() || line.starts_with("//") || line.starts_with("#")) {
            // skip empty lines and comments
            continue;
        }

        if (line.starts_with("+incdir+")) {
            replace(line, "+incdir+", "");
            includeDirs.push_back(line);
            g_compilerManager.addIncludeDir(line);
        } else if (line.starts_with("+define+")) {
            SPDLOG_DEBUG("TODO: +define+ unhandled in F-list");
        } else if (line.starts_with("+")) {
            SPDLOG_WARN("Unknown F-list directive in line: {}", line);
        } else {
            // assume a path
            auto path= std::filesystem::path(line);
            insert(std::filesystem::absolute(path), true);
        }
    }
}
