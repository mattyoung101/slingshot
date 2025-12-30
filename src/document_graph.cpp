// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/document_graph.hpp"
#include <filesystem>
#include <graaflib/algorithm/topological_sorting/dfs_topological_sorting.h>
#include <graaflib/graph.h>
#include <graaflib/types.h>
#include <optional>
#include <spdlog/spdlog.h>
#include <vector>

using namespace slingshot;

void DocumentGraph::insertDocument(const std::filesystem::path &path) {
    SPDLOG_DEBUG("Insert document {} into graph", path.string());
    vertices[path] = graph.add_vertex(path);
    if (!unresolvedSymbols.contains(path)) {
        unresolvedSymbols[path] = std::vector<std::string>();
    }
}

void DocumentGraph::linkDocuments(
    const std::filesystem::path &provider, const std::filesystem::path &requirer, const std::string &symbol) {
    auto providerId = vertices.at(provider);
    auto requirerId = vertices.at(requirer);
    graph.add_edge(providerId, requirerId, symbol);
}

std::optional<std::vector<std::filesystem::path>> DocumentGraph::topologicalSort() {
    std::optional<std::vector<graaf::vertex_id_t>> sorted = graaf::algorithm::dfs_topological_sort(graph);
    if (!sorted.has_value() || sorted == std::nullopt) {
        SPDLOG_ERROR("Failed to perform topological sort of document graph; this graph has cycles!");
        SPDLOG_ERROR("This probably means your project is malformed and has dependency cycles.");
        return std::nullopt;
    }

    std::vector<std::filesystem::path> out;
    out.resize(sorted->size());
    for (const auto &vert : *sorted) {
        auto value = graph.get_vertex(vert);
        out.push_back(value);
    }

    return out;
}

bool DocumentGraph::hasIndexed(const std::filesystem::path &path) {
    // FIXME sloowwww; O(|E|)
    for (const auto &relationship : graph.get_edges()) {
        const auto &[edge, symbol] = relationship;
        const auto &[left, right] = edge;

        const auto leftValue = graph.get_vertex(left);
        const auto rightValue = graph.get_vertex(right);
        if (leftValue == path || rightValue == path) {
            // we say this document has been indexed if there is *at least one* edge that has it on the LHS or
            // RHS
            return true;
        }
    }

    // otherwise, really no edges at all? not indexed!
    return false;
}
