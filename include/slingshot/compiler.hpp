// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include <slang/ast/Compilation.h>
#include <slang/syntax/SyntaxTree.h>
#define BS_THREAD_POOL_NATIVE_EXTENSIONS
#include "BS_thread_pool.hpp"
#include "ankerl/unordered_dense.h"
#include "slang/diagnostics/DiagnosticClient.h"
#include "slang/diagnostics/DiagnosticEngine.h"
#include "slang/diagnostics/Diagnostics.h"
#include "slang/text/SourceManager.h"
#include <filesystem>
#include <lsp/types.h>
#include <memory>
#include <optional>
#include <slang/text/SourceLocation.h>
#include <spdlog/spdlog.h>
#include <string>
#include <utility>
#include <vector>

namespace slingshot {

using namespace slang;

/// A slang::DiagnosticClient that turns Slang diagnostics into LSP diagnostics
class LSPDiagnosticClient : public DiagnosticClient {
public:
    /// @param targetPath The path that we want to report diagnostics for
    LSPDiagnosticClient(std::filesystem::path targetPath)
        : targetPath(std::move(targetPath)) {
    }

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
    std::filesystem::path targetPath;
};

class CompilationManager {
public:
    // TODO update: the *indexing* should run in the thread pool, but the compilation job should just run on
    // its own thread

    /// Submits a compilation job asynchronously
    void submitCompilationJob(const std::string &document, const std::filesystem::path &path);

    /// Submits an indexing job asynchronously; which is slightly different from a compilation job
    void indexDocument(const std::string &document, const std::filesystem::path &path);

    std::optional<Diagnostics> getDiagnostics(const std::filesystem::path &path);

    void addIncludeDir(const std::string &dir) {
        auto err = sourceMgr->addUserDirectories(dir);
        if (err) {
            SPDLOG_ERROR("Failed to add include dir '{}': {}", dir, err.message());
        }
    }

    /// List of files the editor has open
    ankerl::unordered_dense::set<std::filesystem::path> openFiles {};

    /// Association between a FS path and a Slang BufferID once it's been added to the internal SourceManager
    ankerl::unordered_dense::map<std::filesystem::path, BufferID> bufferIds;

    /// Inverse of bufferIds
    ankerl::unordered_dense::map<BufferID, std::filesystem::path> bufferIdsInverse;

    /// Gets the source manager. This is really only a hack to plumb this shit into the completion system.
    std::shared_ptr<SourceManager> getSourceManager() {
        return sourceMgr;
    }

    /// Returns a write (unique) lock on the compiler manager
    [[nodiscard]] auto acquireWriteLock() {
        SPDLOG_TRACE("Attempt to acquire write lock");
        return std::unique_lock<std::shared_mutex>(lock);
    }

    /// Returns a read (shared) lock on the compiler manager
    [[nodiscard]] auto acquireReadLock() {
        SPDLOG_TRACE("Attempt to acquire read lock");
        return std::shared_lock<std::shared_mutex>(lock);
    }

private:
    BS::thread_pool<> pool;
    ankerl::unordered_dense::map<std::filesystem::path, Diagnostics> diags;
    std::shared_ptr<SourceManager> sourceMgr = std::make_shared<SourceManager>();
    std::shared_mutex lock;

    void maybeUpdateIndexingProgress(const std::filesystem::path &path);

    std::shared_ptr<slang::syntax::SyntaxTree> doCstParse(
        const std::filesystem::path &path, const SourceBuffer &buf, DiagnosticEngine &diagEngine);

    std::shared_ptr<ast::Compilation> doAstParse(const std::filesystem::path &path, const SourceBuffer &buf,
        DiagnosticEngine &diagEngine, const std::shared_ptr<slang::syntax::SyntaxTree> &tree);

    void doAnalysis(const SourceBuffer &buf, DiagnosticEngine &diagEngine,
        std::shared_ptr<ast::Compilation> &compilation);

    void doLifting(const std::filesystem::path &path, std::shared_ptr<slang::syntax::SyntaxTree> &tree);

    void issueDiagnostics(const std::filesystem::path &path, const LSPDiagnosticClient::Ptr &diagClient);

    void maybeFinaliseIndexingProgress();
};

} // namespace slingshot
