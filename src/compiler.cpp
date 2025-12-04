// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/compiler.hpp"
#include "slingshot/indexing.hpp"
#include "slingshot/slingshot.hpp"
#include <ankerl/unordered_dense.h>
#include <slang/syntax/SyntaxTree.h>
#include <spdlog/spdlog.h>

using namespace slingshot;
using namespace slang::syntax;

void CompilationManager::submitCompilationJob(
    const std::string &document, const std::filesystem::path &path) {
    SPDLOG_DEBUG("Submitting document {} for compilation", path.string());

    // NOTE this may leak memory
    auto buf = sourceMgr.assignText(document);
    slangBufs[path] = buf;

    pool.detach_task([=, this] {
        // do compilation
        // TODO track the time it takes
        auto tree = SyntaxTree::fromBuffer(buf, sourceMgr);
        SPDLOG_DEBUG("Compiled document {}, got {} diagnostics", path.string(), tree->diagnostics().size());

        g_indexManager.associateParse(path, tree);
    });
}
