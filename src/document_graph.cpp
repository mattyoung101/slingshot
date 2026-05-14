// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/document_graph.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <graaflib/algorithm/cycle_detection/dfs_cycle_detection.h>
#include <graaflib/algorithm/graph_traversal/breadth_first_search.h>
#include <graaflib/algorithm/graph_traversal/depth_first_search.h>
#include <graaflib/algorithm/strongly_connected_components/tarjan.h>
#include <graaflib/algorithm/topological_sorting/dfs_topological_sorting.h>
#include <graaflib/edge.h>
#include <graaflib/graph.h>
#include <graaflib/io/dot.h>
#include <graaflib/types.h>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <optional>
#include <spdlog/fmt/bundled/format.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

using namespace slingshot;

using Graph_t = graaf::directed_graph<std::filesystem::path, std::string>;

void DocumentGraph::insertDocument(const std::filesystem::path &path) {
    SPDLOG_TRACE("Insert document vertex {} into graph", path.string());
    vertices[path] = graph.add_vertex(path);
    invertedVertices[path] = invertedGraph.add_vertex(path);
    hasCyclesCacheValid = false;
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
    invertedGraph.add_edge(requirerId, providerId, symbol);
    hasCyclesCacheValid = false;
}

void DocumentGraph::registerProvidedSymbol(const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_DEBUG("{} ---(PROVIDES SYMBOL)---> '{}'", path.string(), symbol);
    auto it = unresolvedSymbols.begin();
    while (it != unresolvedSymbols.end()) {
        auto unresolved = *it;
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
    unresolvedSymbols.insert(
        UnresolvedSymbol { .lhs = std::nullopt, .rhs = path, .symbol = symbol, .maybe = false });
}

void DocumentGraph::registerMaybeRequiredSymbol(
    const std::filesystem::path &path, const std::string &symbol) {
    SPDLOG_TRACE("{} ---(MAYBE requires SYMBOL)---> '{}'", path.string(), symbol);
    unresolvedSymbols.insert(
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
        const auto &sym = *it;
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

void DocumentGraph::debugLocateCycles() {
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

std::vector<std::filesystem::path> DocumentGraph::locateRequiredDependents(
    const std::filesystem::path &path) {
    if (!invertedVertices.contains(path)) {
        SPDLOG_ERROR("Specified path {} not in graph", path.string());
        return { };
    }

    // we do this by performing a BFS on the inverted graph, which we already built earlier (hopefully)
    auto invertedVertex = invertedVertices[path];
    std::vector<std::filesystem::path> dependents;
    auto edgeCallback = [&](const graaf::edge_id_t &edge) {
        const auto &[lhsId, rhsId] = edge;
        const auto &rhs = invertedGraph.get_vertex(rhsId);

        // this **IS** the right way around (since we're on the inverted graph, remember)
        dependents.push_back(rhs);
    };

    // perform the BFS
    graaf::algorithm::breadth_first_traverse(invertedGraph, invertedVertex, edgeCallback);

    SPDLOG_TRACE("Dependents for {}:", path.string());
    for (const auto &d : dependents) {
        SPDLOG_TRACE("    {}", d.string());
    }

    return dependents;
}

bool DocumentGraph::doesHaveCycles() {
    // cache the value, only recompute it if the graph is mutated
    if (!hasCyclesCacheValid) {
        hasCycles = graaf::algorithm::dfs_cycle_detection(graph);
    }
    return hasCycles;
}

std::vector<std::filesystem::path> DocumentGraph::getAllKnownDocuments() {
    std::vector<std::filesystem::path> out;
    out.reserve(vertices.size());
    for (const auto &[key, value] : vertices) {
        out.push_back(key);
    }
    return out;
}

std::string DocumentGraph::debugDumpOutstandingSymbols() {
    std::string out = "All outstanding symbols:\n";
    for (const auto &sym : unresolvedSymbols) {
        // don't consider maybe symbols, for now
        if (sym.maybe) {
            continue;
        }

        out += fmt::format("{} (required by: {})\n", sym.symbol, toString(sym.rhs));
    }

    return out;
}

void DocumentGraph::purgeMaybeRequiredSymbols() {
    SPDLOG_INFO("Purging 'maybe required' symbols, have {} current unresolved", unresolvedSymbols.size());
    auto it = unresolvedSymbols.begin();
    while (it != unresolvedSymbols.end()) {
        const auto &sym = *it;

        if (sym.maybe) {
            SPDLOG_TRACE("Purging maybe required symbol '{}' wanted by {}", sym.symbol, toString(sym.rhs));
            it = unresolvedSymbols.erase(it);
        } else {
            it++;
        }
    }
}
