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
#include <optional>
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
    // TODO tree?
};

class IndexManager {
public:
    /// Inserts a document with the specified absolute path 'path' and contents 'document'. The document hash
    /// is computed using xxHash64, if the document is already in the index, it will not be inserted.
    void insert(const std::filesystem::path &path, const std::string &document);

    void insert(const std::filesystem::path &path);

    [[nodiscard]] std::optional<IndexEntry> retrieve(const std::filesystem::path &path) const;

    [[nodiscard]] std::optional<IndexEntry> retrieve(const std::filesystem::path &path, uint64_t hash) const;

    /// Recursively walks and indexes files in the given directory
    void walkDir(const std::filesystem::path &path);

    /// Serialises the index to disk. baseDir is the project root directory.
    void flush(const std::filesystem::path &baseDir);

private:
    ankerl::unordered_dense::map<std::filesystem::path, IndexEntry> index;
};

} // namespace slingshot
