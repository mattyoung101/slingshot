// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/handlers.hpp"
#include "slingshot/slingshot.hpp"
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
                    g_indexManager.walkDir(dir);
                }
                g_indexManager.includeDirs = *result;
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
					.textDocumentSync = lsp::TextDocumentSyncOptions{
						.openClose = true,
						// this should probably be incremental in future if we actually see any performance
						// problems but W/E for now
						.change    = lsp::TextDocumentSyncKind::Full,
						.save      = true
					},
					.completionProvider = lsp::CompletionOptions {
                        .triggerCharacters = std::vector<std::string>{".", "`", "[", "{"}
					},
					.diagnosticProvider = lsp::DiagnosticOptions {
						.interFileDependencies = false, // TODO this should eventually be true
						.workspaceDiagnostics = false,
						.identifier = "Slingshot",
					},
				},
				.serverInfo = lsp::InitializeResultServerInfo{
					.name    = "Slingshot",
					.version = "1.0.0"
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
    g_indexManager.insert(params.textDocument.uri.path(), params.textDocument.text);
}

void textDocumentChange(const lsp::notifications::TextDocument_DidChange::Params &&params) {
    SPDLOG_DEBUG("Change document: {}", params.textDocument.uri.path());

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
                    g_indexManager.insert(params.textDocument.uri.path(), event.text);
                }
            },
            change);
    }
}

lsp::requests::TextDocument_Diagnostic::Result textDocumentDiagnostic(
    const lsp::requests::TextDocument_Diagnostic::Params &&params) {
    SPDLOG_DEBUG("Diagnostic info request");
    auto path = params.textDocument.uri.path();

    auto lock = g_indexManager.acquireLock();
    auto result = g_indexManager.retrieve(path);
    if (!result.has_value()) {
        SPDLOG_WARN("Document {} is not in index", path);
        return {};
    }
    if ((*result)->tree == nullptr) {
        SPDLOG_WARN("Document {} has not yet been parsed", path);
        return {};
    }

    lsp::RelatedFullDocumentDiagnosticReport output;
    output.items = (*result)->diagnostics;
    SPDLOG_DEBUG("Returning {} diagnostics", output.items.size());

    return output;
}

} // namespace slingshot::handlers
