// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/handlers.hpp"
#include "slingshot/slingshot.hpp"
#include <lsp/messages.h>
#include <lsp/types.h>
#include <spdlog/spdlog.h>

namespace {
std::optional<std::vector<std::string>> TRIGGER_CHARS {};
}; // namespace

namespace slingshot::handlers {

lsp::requests::Initialize::Result initialise(const lsp::requests::Initialize::Params &&params) {
    SPDLOG_INFO("Received init");

	// FIXME I feel like this is kind of dumb and we could do better
    TRIGGER_CHARS->emplace_back(".");
    TRIGGER_CHARS->emplace_back("`"); // macro
    TRIGGER_CHARS->emplace_back("(");
    TRIGGER_CHARS->emplace_back("[");

    return lsp::requests::Initialize::Result{
				.capabilities = {
					.positionEncoding = lsp::PositionEncodingKind::UTF8,
					.textDocumentSync = lsp::TextDocumentSyncOptions{
						.openClose = true,
						.change    = lsp::TextDocumentSyncKind::Full,
						.save      = true
					},
					.completionProvider = lsp::CompletionOptions {
						.triggerCharacters = TRIGGER_CHARS,
					}
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
}

} // namespace slingshot::handlers
