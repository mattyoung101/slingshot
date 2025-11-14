// Slingshot: A SystemVerilog language server.
//
// Copyright (c) 2025 M. L. Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#include "slingshot/handlers.hpp"
#include "slingshot/slingshot.hpp"
#include <spdlog/spdlog.h>

namespace slingshot::handlers {

lsp::requests::Initialize::Result initialise(const lsp::requests::Initialize::Params &&params) {
    return lsp::requests::Initialize::Result{
				.capabilities = {
					.positionEncoding = lsp::PositionEncodingKind::UTF8,
					.textDocumentSync = lsp::TextDocumentSyncOptions{
						.openClose = true,
						.change    = lsp::TextDocumentSyncKind::Full,
						.save      = true
					},
					.hoverProvider = true,
				},
				.serverInfo = lsp::InitializeResultServerInfo{
					.name    = "Slingshot",
					.version = "1.0.0"
				},
			};
}

void exit(const lsp::notifications::Exit &&params) {
    SPDLOG_INFO("Shutting down");
    slingshot::g_running = false;
}

} // namespace slingshot::handlers
