// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "ankerl/unordered_dense.h"
#include "slingshot/language.hpp"
#include <condition_variable>
#include <cstdint>
#include <filesystem>
#include <lsp/types.h>
#include <memory>
#include <mutex>
#include <optional>
#include <shared_mutex>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/syntax/SyntaxTree.h>
#include <spdlog/spdlog.h>
#include <string>
#include <utility>
#include <vector>

namespace slingshot {

/// Index version that this version of Slingshot is compatible with
constexpr std::string INDEX_VERSION = "1.0.0";

class IndexEntry {
public:
    std::string version = INDEX_VERSION;
    std::string path {};
    uint64_t hash {};

    /// Parse tree
    /// WARNING May be nullptr if not yet parsed
    std::shared_ptr<slang::syntax::SyntaxTree> tree {};

    /// True if the parse tree is valid, false if the parse tree is invalidated and we're waiting a new parse
    bool valid = false;

    /// Current document. Only valid if parse is 'valid' is true.
    std::optional<lang::Document> doc {};

    IndexEntry(std::string path, uint64_t hash)
        : path(std::move(path))
        , hash(hash) {
    }

    /// Invalidates the index entry when the contents of the file has been updated. newHash is the new WyHash
    /// of the file.
    void invalidate(uint64_t newHash) {
        std::lock_guard<std::mutex> lock(mutex);
        SPDLOG_TRACE("Marking index entry {} as invalid with new hash 0x{:X}", path, newHash);
        hash = newHash;
        // BUT importantly, keep the tree; both the parse tree and the lang::Document
        valid = false;
    }

    /// Wakes up all threads waiting on this entry to become valid, and marks it valid
    void makeValid() {
        std::lock_guard<std::mutex> lock(mutex);
        SPDLOG_TRACE("Marking index entry {} as valid", path);
        valid = true;
        cond.notify_all();
    }

    /// Waits until this index entry is valid
    void waitUntilValid() {
        std::unique_lock<std::mutex> lock(mutex);
        SPDLOG_DEBUG("Waiting until index entry {} is valid", path);
        cond.wait(lock, [this] { return valid; });
    }

    /// If this index entry is not valid, waits until it is.
    void ensureValidByWaiting() {
        std::unique_lock<std::mutex> lock(mutex);
        if (!valid) {
            SPDLOG_TRACE("Index entry '{}' is not valid, waiting until it is", path);
            cond.wait(lock, [this] { return valid; });
        } else {
            SPDLOG_TRACE("Index entry '{}' already valid", path);
        }
    }

    using Ptr = std::shared_ptr<IndexEntry>;

private:
    std::mutex mutex {};
    std::condition_variable cond {};
};

class IndexManager {
public:
    /// Inserts a document with the specified absolute path 'path' and contents 'document'. The document hash
    /// is computed using xxHash64, if the document is already in the index, it will not be inserted.
    void insert(const std::filesystem::path &path, const std::string &document);

    void insert(const std::filesystem::path &path);

    /// Associates parse data with a path in the syntax tree
    void associateParse(
        const std::filesystem::path &path, const std::shared_ptr<slang::syntax::SyntaxTree> &tree);

    /// Associates a slingshot::lang::Document with the file
    void associateLangDoc(const std::filesystem::path &path, const lang::Document &doc);

    /// Locate diagnostics. WARNING: Takes a read lock.
    [[nodiscard]] std::optional<IndexEntry::Ptr> retrieve(const std::filesystem::path &path);

    /// Locate diagnostics. WARNING: Takes a read lock.
    [[nodiscard]] std::optional<IndexEntry::Ptr> retrieve(const std::filesystem::path &path, uint64_t hash);

    /// Recursively walks and indexes files in the given directory
    void walkDir(const std::filesystem::path &path);

    /// Serialises the index to disk. baseDir is the project root directory.
    void flush(const std::filesystem::path &baseDir);

    /// Retrieves all syntax trees from all parsed documents
    std::vector<std::shared_ptr<slang::syntax::SyntaxTree>> getAllSyntaxTrees();

    std::string debugDump();

    std::string dumpLangTrees();

    /// Returns a write (unique) lock on the whole index
    [[nodiscard]] auto acquireWriteLock() {
        SPDLOG_TRACE("Attempt to acquire write lock");
        return std::unique_lock<std::shared_mutex>(lock);
    }

    /// Returns a read (shared) lock on the whole index
    [[nodiscard]] auto acquireReadLock() {
        SPDLOG_TRACE("Attempt to acquire write lock");
        return std::shared_lock<std::shared_mutex>(lock);
    }

    std::vector<std::string> includeDirs;

private:
    std::shared_mutex lock;
    ankerl::unordered_dense::map<std::filesystem::path, IndexEntry::Ptr> index;
};

} // namespace slingshot
