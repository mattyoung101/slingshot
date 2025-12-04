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
#include <vector>
#include <yaml-cpp/node/node.h>
#include <yaml-cpp/node/parse.h>

namespace {
std::optional<std::vector<std::string>> TRIGGER_CHARS {};

std::optional<std::vector<std::string>> parseConfigYaml(std::filesystem::path &path) {
    try {
        YAML::Node config = YAML::LoadFile(path.string());

        auto version = config["version"].as<std::string>();
        if (version != slingshot::CONFIG_VERSION) {
            SPDLOG_ERROR("Config version mismatch. Project uses {}, but server uses {}", version,
                slingshot::CONFIG_VERSION);
            return std::nullopt;
        }

        auto includeDirs = config["includeDirs"].as<std::vector<std::string>>();

        return includeDirs;
    } catch (const std::exception &e) {
        SPDLOG_ERROR("Failed to parse config YAML: {}", e.what());
        return std::nullopt;
    }
}

}; // namespace

namespace slingshot::handlers {

lsp::requests::Initialize::Result initialise(const lsp::requests::Initialize::Params &&params) {
    SPDLOG_INFO("Received init");

    // FIXME I feel like this is kind of dumb and we could do better
    TRIGGER_CHARS->emplace_back(".");
    TRIGGER_CHARS->emplace_back("`"); // macro
    TRIGGER_CHARS->emplace_back("(");
    TRIGGER_CHARS->emplace_back("[");

    if (params.rootUri.isNull()) {
        SPDLOG_ERROR("No root URI path specified!");
    } else {
        auto root = params.rootUri->path();
        SPDLOG_INFO("Attempting to locate .slingshot.yaml file in {}", root);

        auto yamlFile = std::filesystem::path(std::string(root) + "/.slingshot.yaml");
        auto ymlFile = std::filesystem::path(std::string(root) + "/.slingshot.yml");

        if (std::filesystem::exists(yamlFile)) {
            // parse it
            auto result = parseConfigYaml(yamlFile);
            if (result == std::nullopt) {
                SPDLOG_ERROR("Failed to parse config YAML. See above.");
            }
        } else if (std::filesystem::exists(ymlFile)) {
            // parse it
            auto result = parseConfigYaml(ymlFile);
            if (result == std::nullopt) {
                SPDLOG_ERROR("Failed to parse config YAML. See above.");
            }
        } else {
            SPDLOG_ERROR("Could not locate .slingshot.yaml file. Index may be non-functional!");
            SPDLOG_ERROR("Tried: {}, {}", yamlFile.string(), ymlFile.string());
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
						.triggerCharacters = TRIGGER_CHARS,
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
    // g_documents[std::string(params.textDocument.uri.path())] = params.textDocument.text;
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
    return {};
}

} // namespace slingshot::handlers
