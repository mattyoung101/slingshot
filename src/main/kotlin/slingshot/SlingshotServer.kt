/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot

import org.eclipse.lsp4j.*
import org.eclipse.lsp4j.jsonrpc.messages.Either
import org.eclipse.lsp4j.services.LanguageServer
import org.eclipse.lsp4j.services.TextDocumentService
import org.eclipse.lsp4j.services.WorkspaceService
import java.util.concurrent.CompletableFuture

/**
 * Instance of the Slingshot language server
 */
class SlingshotServer : LanguageServer {
    private val textDocumentService = SlingshotTextDocumentService()
    private val workspaceService = SlingshotWorkspaceService()

    override fun initialize(params: InitializeParams): CompletableFuture<InitializeResult> {
        val caps = ServerCapabilities()
        // this is the legacy option but hopefully will suffice
        caps.textDocumentSync = Either.forLeft(TextDocumentSyncKind.Full)

        return CompletableFuture.supplyAsync {
            InitializeResult(
                caps,
                ServerInfo(
                    "Slingshot",
                    SLINGSHOT_VERSION
                )
            )
        }
    }

    override fun shutdown(): CompletableFuture<Any> {
        return CompletableFuture.supplyAsync {
            textDocumentService.onShutdown()
        }
    }

    override fun exit() {
        textDocumentService.onShutdown()
    }

    override fun getTextDocumentService(): TextDocumentService {
        return textDocumentService
    }

    override fun getWorkspaceService(): WorkspaceService {
        return workspaceService
    }
}