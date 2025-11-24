// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/indexing.hpp"
#include <ankerl/unordered_dense.h>
#include <spdlog/spdlog.h>

using json = nlohmann::json;
using namespace slingshot;

void to_json(json &j, const IndexEntry &entry) {
    j = json { { "version", entry.version }, { "path", entry.path }, { "hash", entry.hash } };
}

void from_json(const json &j, IndexEntry &p) {
    j.at("version").get_to(p.version);
    j.at("path").get_to(p.path);
    j.at("hash").get_to(p.hash);
}

void IndexManager::insert(const std::filesystem::path &path, const std::string &document) {
    SPDLOG_TRACE("Insert document");

    auto hash = ankerl::unordered_dense::detail::wyhash::hash(document.c_str(), document.size());

    IndexEntry entry { .version = INDEX_VERSION, .path = path, .hash = hash };
}
