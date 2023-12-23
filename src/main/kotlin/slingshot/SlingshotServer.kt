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
import org.eclipse.lsp4j.services.*
import org.tinylog.kotlin.Logger
import java.util.concurrent.CompletableFuture

/**
 * Instance of the Slingshot language server
 */
class SlingshotServer : LanguageServer, LanguageClientAware {
    private val textDocumentService = SlingshotTextDocumentService()
    private val workspaceService = SlingshotWorkspaceService()
    private var client: LanguageClient? = null

    override fun initialize(params: InitializeParams): CompletableFuture<InitializeResult> {
        return CompletableFuture.supplyAsync {
            Logger.info("Client connected: ${params.clientInfo}")

            val caps = ServerCapabilities()
            // this is the legacy option but hopefully will suffice
            caps.textDocumentSync = Either.forLeft(TextDocumentSyncKind.Full)
            caps.completionProvider = CompletionOptions(
                false,
                TRIGGER_CHARACTERS
            )
            // we only provide per-file diagnostics at the moment, and not for the whole "workspace"
            caps.diagnosticProvider = DiagnosticRegistrationOptions(false, false)

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
            Logger.info("LSP shutdown()")
            textDocumentService.onShutdown()
            exit()
        }
    }

    override fun exit() {
        Logger.info("LSP exit()")
        textDocumentService.onShutdown()
        exit()
    }

    override fun getTextDocumentService(): TextDocumentService {
        return textDocumentService
    }

    override fun getWorkspaceService(): WorkspaceService {
        return workspaceService
    }

    override fun connect(client: LanguageClient) {
        Logger.info("Client connected!")
        this.client = client
        textDocumentService.connect(client)
    }

    companion object {
        private val TRIGGER_CHARACTERS = "qwertyuiopasdfghjklzxcvbnm.[".toList().map { it.toString() }
    }
}