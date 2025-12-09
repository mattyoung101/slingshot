// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "BS_thread_pool.hpp"
#include "ankerl/unordered_dense.h"
#include "slang/diagnostics/DiagnosticClient.h"
#include "slang/diagnostics/DiagnosticEngine.h"
#include "slang/diagnostics/Diagnostics.h"
#include "slang/text/SourceManager.h"
#include <condition_variable>
#include <filesystem>
#include <lsp/types.h>
#include <memory>
#include <mutex>
#include <optional>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <vector>

namespace slingshot {

using namespace slang;

/// A slang::DiagnosticClient that turns Slang diagnostics into LSP diagnostics
class LSPDiagnosticClient : public DiagnosticClient {
public:
    auto getLspDiagnostics() {
        return lspDiags;
    }

    void setSourceManager(const std::shared_ptr<SourceManager> &sourceMgr) {
        this->sourceMgr = sourceMgr;
    }

    void report(const ReportedDiagnostic &diagnostic) override;

    using Ptr = std::shared_ptr<LSPDiagnosticClient>;

private:
    std::vector<lsp::Diagnostic> lspDiags;
    std::shared_ptr<SourceManager> sourceMgr;
};

class CompilationManager {
public:
    /// Submits a compilation job asynchronously
    void submitCompilationJob(const std::string &document, const std::filesystem::path &path);

    std::optional<Diagnostics> getDiagnostics(const std::filesystem::path &path);

    void addIncludeDir(const std::string &dir) {
        auto err = sourceMgr->addUserDirectories(dir);
        if (err) {
            SPDLOG_ERROR("Failed to add include dir '{}': {}", dir, err.message());
        }
    }

    void awaitCompilation(const std::filesystem::path &path);

private:
    BS::thread_pool<> pool;
    ankerl::unordered_dense::map<std::filesystem::path, Diagnostics> diags;
    ankerl::unordered_dense::map<std::filesystem::path, SourceBuffer> slangBufs;
    std::shared_ptr<SourceManager> sourceMgr = std::make_shared<SourceManager>();

    std::condition_variable compilationDone;
    std::mutex compilationDoneMtx;
    /// path of the object we just compiled
    std::filesystem::path didJustCompile;
};

} // namespace slingshot
