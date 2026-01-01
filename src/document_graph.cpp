// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/document_graph.hpp"
#include <filesystem>
#include <graaflib/algorithm/topological_sorting/dfs_topological_sorting.h>
#include <graaflib/edge.h>
#include <graaflib/graph.h>
#include <graaflib/io/dot.h>
#include <graaflib/types.h>
#include <iterator>
#include <optional>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

using namespace slingshot;

void DocumentGraph::insertDocument(const std::filesystem::path &path) {
    SPDLOG_DEBUG("Insert document {} into graph", path.string());
    vertices[path] = graph.add_vertex(path);
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

void DocumentGraph::registerProvidedSymbol(const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_DEBUG("Register provided symbol {} by document {}", symbol, path.string());
    auto it = unresolvedSymbols.begin();
    while (it != unresolvedSymbols.end()) {
        auto &unresolved = *it;
        // does this unresolved linking refer to the symbol we have now found?
        if (unresolved.symbol == symbol) {
            // maybe we can resolve some missing things?
            if (unresolved.lhs == std::nullopt) {
                unresolved.lhs = path;
                SPDLOG_DEBUG("Resolved LHS for path {} with symbol {}", path.string(), symbol);
            }
            if (unresolved.rhs == std::nullopt) {
                unresolved.rhs = path;
                SPDLOG_DEBUG("Resolved RHS for path {} with symbol {}", path.string(), symbol);
            }

            // and now, maybe the resolution is complete?
            if (unresolved.lhs != std::nullopt && unresolved.rhs != std::nullopt) {
                SPDLOG_WARN("Completed symbol graph linking: {} ---({})---> {}", unresolved.lhs->string(),
                    symbol, unresolved.rhs->string());
                linkDocuments(*unresolved.lhs, *unresolved.rhs, symbol);
                it = unresolvedSymbols.erase(it);
            } else {
                it++;
            }
        } else {
            it++;
        }
    }
}

void DocumentGraph::registerRequiredSymbol(const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_DEBUG("Register required symbol {} by document {}", symbol, path.string());
    unresolvedSymbols.push_back(UnresolvedSymbol { .lhs = std::nullopt, .rhs = path, .symbol = symbol });
}

void DocumentGraph::dumpDot() {
    const auto vertex_writer { [](graaf::vertex_id_t vertex_id,
                                   const std::filesystem::path &vertex) -> std::string {
        return fmt::format("label=\"{}: {}\"", vertex_id, vertex.string());
    } };

    const auto edge_writer { [](const graaf::edge_id_t &edge_id, const std::string &edge) -> std::string {
        return fmt::format("label=\"{}\"", edge);
    } };

    graaf::io::to_dot(graph, "/tmp/slingshot_document_graph.dot", vertex_writer, edge_writer);
}
