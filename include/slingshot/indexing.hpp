// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "ankerl/unordered_dense.h"
#include "nlohmann/json.hpp"
#include <cstdint>
#include <filesystem>
#include <lsp/types.h>
#include <memory>
#include <mutex>
#include <optional>
#include <slang/diagnostics/Diagnostics.h>
#include <slang/syntax/SyntaxTree.h>
#include <string>
#include <vector>

namespace slingshot {

/// Index version that this version of Slingshot is compatible with
constexpr std::string INDEX_VERSION = "1.0.0";

class IndexEntry {
public:
    std::string version;
    std::string path;
    uint64_t hash;

    /// Parse tree
    /// WARNING May be nullptr if not yet parsed
    std::shared_ptr<slang::syntax::SyntaxTree> tree;

    /// Processed LSP diagnostics, only really valid if tree != nullptr
    std::vector<lsp::Diagnostic> diagnostics;

    using Ptr = std::shared_ptr<IndexEntry>;
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

    /// Associates processed diagnotsic data from the LSPDiagnosticClient in compiler.hpp
    void associateDiagnostics(
        const std::filesystem::path &path, const std::vector<lsp::Diagnostic> &diagnostics);

    [[nodiscard]] std::optional<IndexEntry::Ptr> retrieve(const std::filesystem::path &path) const;

    [[nodiscard]] std::optional<IndexEntry::Ptr> retrieve(
        const std::filesystem::path &path, uint64_t hash) const;

    /// Recursively walks and indexes files in the given directory
    void walkDir(const std::filesystem::path &path);

    /// Serialises the index to disk. baseDir is the project root directory.
    void flush(const std::filesystem::path &baseDir);

    std::string debugDump();

    std::vector<std::string> includeDirs;

    [[nodiscard]] auto acquireLock() {
        return std::lock_guard<std::mutex>(lock);
    }

private:
    std::mutex lock;
    ankerl::unordered_dense::map<std::filesystem::path, IndexEntry::Ptr> index;
};

} // namespace slingshot
