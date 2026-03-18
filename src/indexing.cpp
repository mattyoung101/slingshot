// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/indexing.hpp"
#include "slingshot/compiler.hpp"
#include "slingshot/language.hpp" // NECESSARY for JSON conversion
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <cerrno>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <lsp/json/json.h>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <memory>
#include <nlohmann/json_fwd.hpp>
#include <optional>
#include <slang/parsing/KnownSystemName.h>
#include <slang/syntax/SyntaxTree.h>
#include <spdlog/spdlog.h>
#include <sstream>
#include <string>
#if SLINGSHOT_ENABLE_INOTIFY
#include <sys/inotify.h>
#endif

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

#if SLINGSHOT_ENABLE_INOTIFY
    auto absPath = std::filesystem::absolute(path);
    int wd = inotify_add_watch(inotifyFd, absPath.c_str(), IN_CREATE | IN_DELETE);
    if (wd < 0) {
        SPDLOG_ERROR(
            "Failed to add inotify watch descriptor for path {}: {}", absPath.string(), strerror(errno));
        SPDLOG_DEBUG("WHY: wd: {}, fd: {}", wd, inotifyFd);
    } else {
        inotifyWd = wd;
        SPDLOG_DEBUG("Added an inotify watcher, wd: {}, fd: {}", wd, inotifyFd);
    }
#endif
}

ankerl::unordered_dense::set<std::shared_ptr<slang::syntax::SyntaxTree>> IndexManager::getAllSyntaxTrees() {
    auto lock = acquireLock();
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

void IndexManager::inotifyWatchThread() {
#if SLINGSHOT_ENABLE_INOTIFY
    SPDLOG_DEBUG("Enter inotifyWatchThread()");

    // based on https://gist.github.com/wh5a/302353

    constexpr auto EVENT_SIZE = (sizeof(struct inotify_event));
    constexpr auto BUF_LEN = (1024 * (EVENT_SIZE + 16));

    char buffer[BUF_LEN] = { 0 }; // NOLINT: modernize-avoid-c-arrays

    while (true) {
        long i = 0;
        long length = read(inotifyFd, buffer, BUF_LEN); // NOLINT
        if (length < 0) {
            SPDLOG_ERROR("Failed to read from inotify fd: {}", strerror(errno));
            return;
        }

        while (i < length) {
            auto *event = (struct inotify_event *) &buffer[i];
            if (event->len != 0u) {
                if ((event->mask & IN_CREATE) != 0u) {
                    if ((event->mask & IN_ISDIR) != 0u) {
                        SPDLOG_INFO("Dir created: {}", event->name);
                    } else {
                        SPDLOG_INFO("File created: {}", event->name);
                    }
                } else if ((event->mask & IN_DELETE) != 0u) {
                    if ((event->mask & IN_ISDIR) != 0u) {
                        SPDLOG_INFO("Dir deleted: {}", event->name);
                    } else {
                        SPDLOG_INFO("File deleted: {}", event->name);
                    }
                }
            }
            i += EVENT_SIZE + event->len;
        }
    }

#endif
}

void IndexManager::maybeInitialiseInotify() {
#if SLINGSHOT_ENABLE_INOTIFY
    SPDLOG_INFO("Setting up inotify");
    inotifyFd = inotify_init();
    if (inotifyFd < 0) {
        SPDLOG_ERROR("Failed to initialise inotify: {}", strerror(errno));
    } else {
        SPDLOG_INFO("Inotify init OK");
    }

    auto thread = std::thread(&IndexManager::inotifyWatchThread, this);
    pthread_setname_np(thread.native_handle(), "Inotify");
    thread.detach();
#endif
}
