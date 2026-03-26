// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/document_graph.hpp"
#include "slingshot/language.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <cstdint>
#include <filesystem>
#include <graaflib/algorithm/graph_traversal/breadth_first_search.h>
#include <graaflib/algorithm/strongly_connected_components/tarjan.h>
#include <graaflib/algorithm/topological_sorting/dfs_topological_sorting.h>
#include <graaflib/edge.h>
#include <graaflib/graph.h>
#include <graaflib/io/dot.h>
#include <graaflib/types.h>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <optional>
#include <spdlog/spdlog.h>
#include <string>
#include <tuple>
#include <vector>

using namespace slingshot;

using Graph_t = graaf::directed_graph<std::filesystem::path, std::string>;

template <>
struct ankerl::unordered_dense::hash<std::vector<graaf::vertex_id_t>> {
    using is_avalanching = void;

    [[nodiscard]] auto operator()(std::vector<graaf::vertex_id_t> const &x) const noexcept -> uint64_t {
        uint64_t hash = 0xBEEF;
        for (const auto &elem : x) {
            uint64_t update = detail::wyhash::hash(elem);
            hash = detail::wyhash::mix(hash, update);
        }
        return hash;
    }
};

template <>
struct ankerl::unordered_dense::hash<std::vector<graaf::edge_id_t>> {
    using is_avalanching = void;

    [[nodiscard]] auto operator()(std::vector<graaf::edge_id_t> const &x) const noexcept -> uint64_t {
        uint64_t hash = 0xBEEF;
        for (const auto &elem : x) {
            uint64_t update
                = detail::wyhash::mix(detail::wyhash::hash(elem.first), detail::wyhash::hash(elem.second));
            hash = detail::wyhash::mix(hash, update);
        }
        return hash;
    }
};

void DocumentGraph::insertDocument(const std::filesystem::path &path) {
    SPDLOG_TRACE("Insert document vertex {} into graph", path.string());
    vertices[path] = graph.add_vertex(path);
}

void DocumentGraph::linkDocuments(
    const std::filesystem::path &provider, const std::filesystem::path &requirer, const std::string &symbol) {
    auto providerId = vertices.at(provider);
    auto requirerId = vertices.at(requirer);
    // HACK: this isn't the smartest way to solve this problem, but it's impossible for a document to depend
    // on itself, so reject self-referential edges
    if (providerId == requirerId) {
        SPDLOG_WARN("Rejecting self-edge on vetex {} symbol {}", provider.string(), symbol);
        return;
    }
    graph.add_edge(providerId, requirerId, symbol);
}

std::optional<std::vector<std::filesystem::path>> DocumentGraph::topologicalSort() {
    std::optional<std::vector<graaf::vertex_id_t>> sorted = graaf::algorithm::dfs_topological_sort(graph);
    if (!sorted.has_value()) {
        SPDLOG_ERROR("Failed to perform topological sort of document graph; this graph has cycles!");
        SPDLOG_ERROR("This probably means your project is malformed and has dependency cycles.");

        // identify and print the cycle for debugging
        locateCycles();

        return std::nullopt;
    }

    std::vector<std::filesystem::path> out;
    out.reserve(sorted->size());
    for (const auto &vert : *sorted) {
        auto value = graph.get_vertex(vert);
        out.push_back(value);
    }

    return out;
}

void DocumentGraph::registerProvidedSymbol(const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_DEBUG("{} ---(PROVIDES SYMBOL)---> '{}'", path.string(), symbol);
    auto it = unresolvedSymbols.begin();
    while (it != unresolvedSymbols.end()) {
        auto &unresolved = *it;
        // does this unresolved linking refer to the symbol we have now found?
        if (unresolved.symbol == symbol) {
            // maybe we can resolve some missing things?
            if (unresolved.lhs == std::nullopt) {
                unresolved.lhs = path;
                SPDLOG_TRACE("Resolved LHS for path {} with symbol {}", path.string(), symbol);
            }
            if (unresolved.rhs == std::nullopt) {
                unresolved.rhs = path;
                SPDLOG_TRACE("Resolved RHS for path {} with symbol {}", path.string(), symbol);
            }

            // and now, maybe the resolution is complete?
            if (unresolved.lhs != std::nullopt && unresolved.rhs != std::nullopt) {
                SPDLOG_INFO("Completed symbol graph linking: {} ---({})---> {}", unresolved.lhs->string(),
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
    symbolProviders[path].push_back(symbol);
}

void DocumentGraph::registerRequiredSymbol(const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_TRACE("{} ---(REQUIRES SYMBOL)---> '{}'", path.string(), symbol);
    unresolvedSymbols.push_back(
        UnresolvedSymbol { .lhs = std::nullopt, .rhs = path, .symbol = symbol, .maybe = false });
}

void DocumentGraph::registerMaybeRequiredSymbol(
    const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_TRACE("{} ---(MAYBE requires SYMBOL)---> '{}'", path.string(), symbol);
    unresolvedSymbols.push_back(
        UnresolvedSymbol { .lhs = std::nullopt, .rhs = path, .symbol = symbol, .maybe = true });
}

void DocumentGraph::dumpDot() {
    const auto vertex_writer { [](graaf::vertex_id_t vertex_id,
                                   const std::filesystem::path &vertex) -> std::string {
        return fmt::format("label=\"{}: {}\"", vertex_id, vertex.string());
    } };

    const auto edge_writer { [](const graaf::edge_id_t &, const std::string &edge) -> std::string {
        return fmt::format("label=\"{}\"", edge);
    } };

    graaf::io::to_dot(graph, "/tmp/slingshot_document_graph.dot", vertex_writer, edge_writer, true);
}

void DocumentGraph::finaliseOutstandingSymbols() {
    auto it = unresolvedSymbols.begin();
    while (it != unresolvedSymbols.end()) {
        auto &sym = *it;
        SPDLOG_TRACE("Trying to finalise outstanding symbol '{}': LHS '{}', RHS '{}'", sym.symbol,
            toString(sym.lhs), toString(sym.rhs));

        if (!sym.lhs.has_value()) {
            if (!sym.rhs.has_value()) {
                SPDLOG_WARN("RHS does not have a value (bugprone), skipping");
                it++;
                continue;
            }

            // see if we can find a resolver for this symbol in the graph
            auto provider = findProvider(sym.symbol);
            // ensure also that we're not creating self loops
            if (provider.has_value() && *provider != sym.rhs) {
                SPDLOG_DEBUG("Found provider for symbol '{}': '{}'", sym.symbol, provider->string());
                linkDocuments(*provider, *sym.rhs, sym.symbol);
                it = unresolvedSymbols.erase(it);
            } else {
                if (sym.maybe) {
                    SPDLOG_TRACE(
                        "Could not immediately find resolver for MAYBE required symbol: '{}' - removing it",
                        sym.symbol);
                    it = unresolvedSymbols.erase(it);
                    continue;
                }
                SPDLOG_TRACE("Could NOT provide provider for unresolved symbol '{}' wanted by '{}'",
                    sym.symbol, sym.rhs->string());
                it++;
            }
        } else {
            // if the unresolved part is on the RHS, we don't handle that yet; doesn't seem to come up in
            // practice much
            SPDLOG_WARN("Unresolved symbol '{}' has an unresolved RHS, which is unhandled", sym.symbol);
            it++;
        }
    }
}

std::optional<std::filesystem::path> DocumentGraph::findProvider(const std::string &symbol) {
    for (const auto &pair : symbolProviders) {
        const auto &[doc, symbols] = pair;
        for (const auto &s : symbols) {
            if (s == symbol) {
                // this document provides the symbol we want: we found a provider
                return doc;
            }
        }
    }
    // no luck
    return std::nullopt;
}

void DocumentGraph::locateCycles() {
    // this method is probably not perfect, i'm not a 1337 leetcoder sorry
    auto sccs = graaf::algorithm::tarjans_strongly_connected_components(graph);
    SPDLOG_ERROR("Graph has {} SCCs", sccs.size());
    for (const auto &scc : sccs) {
        SPDLOG_ERROR("SCC has size {}", scc.size());
        for (size_t i = 0; i < scc.size(); i++) {
            SPDLOG_ERROR("    {}. {}", i, scc[i]);
            // TODO locate the edge that connects this node
        }
    }
}

std::vector<Graph_t> DocumentGraph::determineSubGraphs() {
    // a set of all the unique subgraphs; which we store as edges. we can then reconstruct the actual graph
    // later
    ankerl::unordered_dense::set<std::vector<graaf::edge_id_t>> subgraphs;

    // do BFS traversal for each node to find all the graphs
    for (const auto &[vertId, vert] : graph.get_vertices()) {
        std::vector<graaf::edge_id_t> allEdges;
        // record all the edges in this subgraph
        auto edgeCallback = [&allEdges](const graaf::edge_id_t &edge) { allEdges.push_back(edge); };
        graaf::algorithm::breadth_first_traverse(graph, vertId, edgeCallback);

        // insert into the subgraph; this is fine since it's a set which means it's unique right
        subgraphs.insert(allEdges);
    }

    SPDLOG_DEBUG("Found {} unique subgraphs", subgraphs.size());

    std::vector<Graph_t> out;
    for (const auto &subgraphEdges : subgraphs) {
        SPDLOG_ERROR("Subgraph has size {} edges", subgraphEdges.size());

        Graph_t subgraph;

        for (const auto &[lhsId, rhsId] : subgraphEdges) { }
    }

    return out;
}
