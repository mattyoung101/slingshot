// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <ankerl/unordered_dense.h>
#include <filesystem>
#include <graaflib/graph.h>
#include <graaflib/types.h>
#include <optional>
#include <string>
#include <vector>

namespace slingshot {

/// The document graph is a DAG that maps relationships between documents and their dependencies on other
/// documents via the symbols they use.
class DocumentGraph {
public:
    void insertDocument(const std::filesystem::path &path);

    /// Links the document that provides the symbol ("provider"/"A") to the document that requires the symbol
    /// ("requirer"/B). This creates the edge A ---(sym)--> B.
    void linkDocuments(const std::filesystem::path &provider, const std::filesystem::path &requirer,
        const std::string &symbol);

    /// Performs a topological sort of the document graph if possible.
    std::optional<std::vector<std::filesystem::path>> topologicalSort();

    /// Returns true if we have indexed and built relationships using this particular path
    bool hasIndexed(const std::filesystem::path &path);

    /// Given a symbol and a document the symbol is from, queries the list of unresolved symbols and resolve
    /// any outstanding symbols
    void registerProvidedSymbol(const std::filesystem::path &path, const std::string &symbol);

    void registerRequiredSymbol(const std::filesystem::path &path, const std::string &symbol);

    /// Dumps the graph to a DOT file
    void dumpDot();

private:
    struct UnresolvedSymbol {
        /// LHS, this side provides the symbol
        std::optional<std::filesystem::path> lhs;
        /// RHS, this side requires the symbol
        std::optional<std::filesystem::path> rhs;
        std::string symbol;
    };

    /// the actual graph data structure, vertices are paths, and edges are the symbols that are inherited from
    /// these documents.
    /// the direction of the vertex A ---(sym)--> B means that A provides the symbol "sym" **TO** B.
    /// it's like this so that we get the result we want from a topo sort.
    graaf::directed_graph<std::filesystem::path, std::string> graph {};

    /// graaflib makes use vertex IDs, so we store a mapping of vertices to paths here
    ankerl::unordered_dense::map<std::filesystem::path, graaf::vertex_id_t> vertices {};

    std::vector<UnresolvedSymbol> unresolvedSymbols {};
};

} // namespace slingshot
