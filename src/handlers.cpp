// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025-2026 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/handlers.hpp"
#include "slingshot/slingshot.hpp"
#include <algorithm>
#include <chrono>
#include <exception>
#include <filesystem>
#include <lsp/messages.h>
#include <lsp/types.h>
#include <optional>
#include <spdlog/spdlog.h>
#include <toml++/toml.hpp>
#include <vector>

namespace {
std::optional<std::vector<std::string>> parseConfigToml(std::filesystem::path &path) {
    try {
        auto config = toml::parse_file(path.string());

        if (!config.contains("version") || !config.contains("include_dirs")) {
            SPDLOG_ERROR("Configuration is missing version or include_dirs keys");
            return std::nullopt;
        }

        auto *version = config["version"].as_string();
        if (*version != slingshot::CONFIG_VERSION) {
            SPDLOG_ERROR("Config version mismatch. Project uses {}, but server uses {}",
                std::string(*version), slingshot::CONFIG_VERSION);
            return std::nullopt;
        }

        auto *include_dirs = config["include_dirs"].as_array();
        std::vector<std::string> out;
        for (const auto &dir : *include_dirs) {
            out.emplace_back(*dir.as_string());
        }

        return out;
    } catch (const std::exception &e) {
        SPDLOG_ERROR("Failed to parse config toml: {}", e.what());
        return std::nullopt;
    }
}

}; // namespace

namespace slingshot::handlers {

lsp::requests::Initialize::Result initialise(const lsp::requests::Initialize::Params &&params) {
    SPDLOG_INFO("Received init");

    if (params.rootUri.isNull()) {
        SPDLOG_ERROR("No root URI path specified!");
    } else {
        auto root = params.rootUri->path();
        SPDLOG_INFO("Attempting to locate .slingshot.toml file in {}", root);

        auto tomlFile = std::filesystem::path(std::string(root) + "/.slingshot.toml");

        if (std::filesystem::exists(tomlFile)) {
            // parse it
            auto result = parseConfigToml(tomlFile);
            if (result == std::nullopt) {
                SPDLOG_ERROR("Failed to parse config toml. See above.");
            } else {
                // we have the index file
                SPDLOG_INFO("Config TOML parsed successfully");
                for (const auto &dir : *result) {
                    g_compilerManager.addIncludeDir(dir);
                }
                g_indexManager.includeDirs = *result;

                // **now** that we've registered the include dirs, we can actually walk the directories
                for (const auto &dir : *result) {
                    g_indexManager.walkDir(dir);
                }
            }
        } else {
            SPDLOG_ERROR("Could not locate .slingshot.toml file. Index may be non-functional!");
            SPDLOG_ERROR("Tried: {}", tomlFile.string());
            // TODO warn client
        }
    }

    return lsp::requests::Initialize::Result{
				.capabilities = {
					.positionEncoding = lsp::PositionEncodingKind::UTF8,
					.textDocumentSync = lsp::TextDocumentSyncOptions {
						.openClose = true,
						// this should probably be incremental in future if we actually see any performance
						// problems but W/E for now
						.change    = lsp::TextDocumentSyncKind::Full,
						.save      = true,
					},
					.completionProvider = lsp::CompletionOptions {
                        .triggerCharacters = std::vector<std::string>{".", "`", "[", "{"},
					},
					.diagnosticProvider = lsp::DiagnosticOptions {
						.interFileDependencies = false, // TODO this should eventually be true
						.workspaceDiagnostics = false,
						.identifier = "Slingshot",
					},
				},
				.serverInfo = lsp::InitializeResultServerInfo{
					.name    = "Slingshot",
					.version = SLINGSHOT_VERSION,
				},
			};
}

void exit() {
    SPDLOG_INFO("Shutting down");
    slingshot::g_running = false;
}

lsp::requests::Shutdown::Result shutdown() {
    SPDLOG_INFO("Shutting down");
    slingshot::g_running = false;
    return lsp::requests::Shutdown::Result {};
}

void textDocumentOpen(const lsp::notifications::TextDocument_DidOpen::Params &&params) {
    SPDLOG_DEBUG("Open document: {}", params.textDocument.uri.path());

    // register in the document database
    g_indexManager.insert(
        std::filesystem::absolute(params.textDocument.uri.path()), params.textDocument.text, false);
    auto lock = g_compilerManager.acquireLock();
    g_compilerManager.openFiles.insert(std::filesystem::absolute(params.textDocument.uri.path()));
}

void textDocumentClose(const lsp::notifications::TextDocument_DidClose::Params &&params) {
    SPDLOG_DEBUG("Close document: {}", params.textDocument.uri.path());
    auto lock = g_compilerManager.acquireLock();
    g_compilerManager.openFiles.erase(std::filesystem::absolute(params.textDocument.uri.path()));
}

void textDocumentChange(const lsp::notifications::TextDocument_DidChange::Params &&params) {
    for (const lsp::TextDocumentContentChangeEvent &change : params.contentChanges) {
        std::visit(
            [&](auto &&arg) {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, lsp::TextDocumentContentChangeEvent_Range_Text>) {
                    // we specifically told the client to send us full text, so if they're sending this, get
                    // mad at them
                    SPDLOG_ERROR("We don't yet handle Event_Range_Text. Your LSP client is borked.");
                }
                if constexpr (std::is_same_v<T, lsp::TextDocumentContentChangeEvent_Text>) {
                    const lsp::TextDocumentContentChangeEvent_Text &event = arg;
                    SPDLOG_TRACE("===== UPDATE {} =====\n{}\n", params.textDocument.uri.path(), event.text);
                    g_indexManager.insert(
                        std::filesystem::absolute(params.textDocument.uri.path()), event.text, false);
                }
            },
            change);
    }
}

void textDocumentSave(const lsp::notifications::TextDocument_DidSave::Params &&params) {
    SPDLOG_TRACE("Did save document: {}", params.textDocument.uri.path());

    if (params.text.has_value()) {
        // register in the document database
        g_indexManager.insert(std::filesystem::absolute(params.textDocument.uri.path()), *params.text, false);
    }
}

lsp::requests::TextDocument_Completion::Result textDocumentCompletion(
    const lsp::requests::TextDocument_Completion::Params &&params) {
    auto path = std::filesystem::absolute(params.textDocument.uri.path()).string();
    SPDLOG_TRACE("Completion request in {}", path);

    auto lock = g_indexManager.acquireLock();
    auto result = g_indexManager.retrieve(path);
    if (!result.has_value()) {
        SPDLOG_WARN("Document {} is not in index", path);
        return {};
    }
    if ((*result)->tree == nullptr) {
        SPDLOG_WARN("Document {} has no parse tree at all, can't do completion", path);
        return {};
    }

    // ensure the index entry is valid
    lock.unlock();
    (*result)->ensureValidByWaiting();

    return CompletionManager::getCompletions(path, params.position, *result);
}

lsp::requests::TextDocument_Diagnostic::Result textDocumentDiagnostic(
    const lsp::requests::TextDocument_Diagnostic::Params &&params) {
    auto compilerLock = g_compilerManager.acquireLock();
    auto indexerLock = g_indexManager.acquireLock();

    if (g_indexManager.isInitialIndexInProgress) {
        SPDLOG_DEBUG("Refusing to issue diagnostics while indexing is in progress");
        // FIXME request diagnostics later
        return lsp::RelatedUnchangedDocumentDiagnosticReport{};
    }

    if (!g_compilerManager.openFiles.contains(params.textDocument.uri.path())) {
        SPDLOG_TRACE("Document {} is not open, skip issuing diagnostics", params.textDocument.uri.path());
        return lsp::RelatedUnchangedDocumentDiagnosticReport{};
    }

    // find documents related to the text document specified
    std::vector<TimestampedDiagnostics> relatedDiagnostics;
    for (const auto &diag : g_compilerManager.outgoingDiagnostics) {
        if (diag.path == params.textDocument.uri.path()) {
            relatedDiagnostics.push_back(diag);
        }
    }

    SPDLOG_TRACE("Found {} related diagnostics out of {} for path {}", relatedDiagnostics.size(),
        g_compilerManager.outgoingDiagnostics.size(), params.textDocument.uri.path());

    if (relatedDiagnostics.empty()) {
        return {};
    }

    // find the most recent diagnostic
    // NOLINTNEXTLINE(modernize-use-ranges) not supported on older compilers
    std::sort(relatedDiagnostics.begin(), relatedDiagnostics.end(),
        [](const auto &a, const auto &b) { return a.timestamp < b.timestamp; });

    lsp::RelatedFullDocumentDiagnosticReport out;
    out.items = relatedDiagnostics[0].lspDiags->getLspDiagnostics();

    // FIXME we should remove only the relatedDiagnostics, not everything
    g_compilerManager.outgoingDiagnostics.clear();

    return out;
}

} // namespace slingshot::handlers
