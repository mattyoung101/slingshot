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
import org.eclipse.lsp4j.services.LanguageClient
import org.eclipse.lsp4j.services.LanguageClientAware
import org.eclipse.lsp4j.services.TextDocumentService
import org.tinylog.kotlin.Logger
import slingshot.completion.ANTLRCompletion
import slingshot.completion.CompletionException
import slingshot.completion.CompletionProvider
import slingshot.diagnostics.DiagnosticProvider
import slingshot.diagnostics.VerilatorDiagnostics
import slingshot.indexing.IndexManager
import java.net.URI
import java.nio.file.Path
import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executors
import java.util.concurrent.ThreadPoolExecutor
import kotlin.io.path.toPath

class SlingshotTextDocumentService : TextDocumentService, LanguageClientAware {
    private val indexManager = IndexManager()
    private val completion: CompletionProvider = ANTLRCompletion()
    private val diagnostics: DiagnosticProvider = VerilatorDiagnostics()
    private var client: LanguageClient? = null
    private val executor = Executors.newCachedThreadPool()

    /** Called when the LSP is shutting down. May be called more than once. */
    fun onShutdown() {
        Logger.info("LSP shutting down")
        // nothing at the moment, but we would flush the index
    }

    /**
     * Called whenever the text document was changed
     */
    private fun changed(path: Path, document: String) {
        // store document in index when it's changed
        // note that we run this _in sync_, not async, because we need to make sure that the text document
        // is available in the index if diagnostic() or completion() is called
        indexManager.insert(path, document)

        // now that we know the document is definitely in the index, run diagnostics asynchronously in the
        // thread pool
        executor.submit {
            val diagnostics = diagnostics.diagnose(path, document)
            client?.publishDiagnostics(PublishDiagnosticsParams(path.toUri().toString(), diagnostics))
        }
    }

    override fun completion(position: CompletionParams): CompletableFuture<Either<MutableList<CompletionItem>, CompletionList>> {
        Logger.debug("completion()")

        return CompletableFuture.supplyAsync {
            return@supplyAsync Either.forLeft(mutableListOf())
        }
    }

    override fun didOpen(params: DidOpenTextDocumentParams) {
        val uri = URI(params.textDocument.uri).toPath()
        val document = params.textDocument.text
        changed(uri, document)
    }

    override fun didChange(params: DidChangeTextDocumentParams) {
        val uri = URI(params.textDocument.uri).toPath()
        val document = params.contentChanges[0].text
        changed(uri, document)
    }

    override fun didClose(params: DidCloseTextDocumentParams) {
    }

    override fun didSave(params: DidSaveTextDocumentParams) {
        val uri = URI(params.textDocument.uri).toPath()
        val document = params.text
        changed(uri, document)
    }

    override fun connect(client: LanguageClient) {
        Logger.debug("Client connected in text document service")
        this.client = client
    }
}