// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#pragma once
#include "moodycamel/concurrentqueue.h"
#include <atomic>
#include <cstdint>
#include <mutex>
#include <slang/ast/Compilation.h>
#include <slang/syntax/SyntaxTree.h>
#include <slang/util/Util.h>
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
#include <moodycamel/blockingconcurrentqueue.h>
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

class DummyDiagnosticClient : public DiagnosticClient {
public:
    void report(const ReportedDiagnostic &diagnostic) override { };
};

class TimestampedDiagnostics {
public:
    /// Time at which these diagnostics were generated
    uint64_t timestamp;

    std::filesystem::path path;

    /// Diagnostics
    LSPDiagnosticClient::Ptr lspDiags;
};

class CompilationManager {
public:
    /// Starts the outgoing diagnostics thread
    void startOutgoingDiagnostics();

    /// Submits a compilation job asynchronously
    void submitCompilationJob(const std::string &document, const std::filesystem::path &path, bool isIndex);

    /// Registers a user specified include dir in the source manager. Does not directly act on this, just
    /// updates the source manager.
    void addIncludeDir(const std::string &dir) {
        auto err = sourceMgr->addUserDirectories(dir);
        if (err) {
            SPDLOG_ERROR("Failed to add include dir '{}': {}", dir, err.message());
        }
    }

    /// List of files the editor has open
    ankerl::unordered_dense::set<std::filesystem::path> openFiles { };

    /// Gets the source manager. This is really only a hack to plumb this shit into the completion system.
    std::shared_ptr<SourceManager> getSourceManager() {
        return sourceMgr;
    }

    /// Returns a lock on the compiler manager
    [[nodiscard]] auto acquireLock() {
        SPDLOG_TRACE("Attempt to acquire lock");
        return std::unique_lock(lock);
    }

    /// Used by the remote debugger to get deps for a file
    std::string debugGetDepsForFile(const std::string &path);

    /// Association between a FS path and a Slang BufferID once it's been added to the internal SourceManager
    ankerl::unordered_dense::map<std::filesystem::path, BufferID> bufferIds;

    friend class LSPDiagnosticClient;

private:
    BS::thread_pool<> pool { };
    ankerl::unordered_dense::map<std::filesystem::path, uint64_t> importHashes;
    ankerl::unordered_dense::map<std::filesystem::path, SourceBuffer> bufMap;
    std::shared_ptr<SourceManager> sourceMgr = std::make_shared<SourceManager>();
    std::recursive_mutex lock;
    std::atomic_int indexingJobsInProgress;

    /// mapping of a document to all the documents it requires to build the AST
    ankerl::unordered_dense::map<std::filesystem::path, std::vector<std::filesystem::path>> requiredDocuments;

    /// mapping between a document and the hash of its Imports calculated by the import locator

    /// Inverse of bufferIds
    ankerl::unordered_dense::map<BufferID, std::filesystem::path> bufferIdsInverse;

    /// outgoing, timestamped diagnostics
    moodycamel::BlockingConcurrentQueue<TimestampedDiagnostics> outgoingDiagnostics { };

    /// Performs a bulk compilation of all the documents in the index, once the document graph has been built
    void performBulkCompilation(bool shouldSendLspNotification);

    /// Fills in the table `requiredDocuments` by analysing the document graph using a BFS
    void locateAllRequiredDocuments(bool shouldSendLspNotification);

    /// Parse the Concrete Syntax Tree. This is essential for all the other routines.
    std::shared_ptr<slang::syntax::SyntaxTree> doCstParse(
        const std::filesystem::path &path, const SourceBuffer &buf, DiagnosticEngine &diagEngine);

    /// Parse the Abstract Syntax Tree. This is only used for diagnostics.
    std::shared_ptr<ast::Compilation> doAstParse(const std::filesystem::path &path, const SourceBuffer &buf,
        DiagnosticEngine &diagEngine, const std::shared_ptr<slang::syntax::SyntaxTree> &tree);

    /// Perform additional analysis. Requires CST and AST parse to be done.
    void doAnalysis(const SourceBuffer &buf, DiagnosticEngine &diagEngine,
        std::shared_ptr<ast::Compilation> &compilation);

    /// Perform custom lang lifting. Necessary for completion.
    void doLifting(const std::filesystem::path &path, std::shared_ptr<slang::syntax::SyntaxTree> &tree);

    /// Issue collected Slang diagnostics to the LSP client. **This should only be called by the outgoing
    /// diagnostics thread!**
    void issueDiagnostics(const std::filesystem::path &path, const LSPDiagnosticClient::Ptr &diagClient);

    /// Checks if indexing is done using thread safe variables. If yes, submits a bulk compilation job.
    void maybeFinaliseIndexingProgress();

    /// If the import table for a file has changed, re-scan the document to see what changed and update the
    /// document graph, and requires documents.
    void reIndexDocument(const std::filesystem::path &path,
        const std::shared_ptr<slang::syntax::SyntaxTree> &tree, bool shouldSendLspNotification);

    /// Recompiles a document that's already in the index, used by reIndexDocument
    void reCompileDocument(const std::filesystem::path &path);

    void outgoingDiagnosticsThread();
};

} // namespace slingshot
