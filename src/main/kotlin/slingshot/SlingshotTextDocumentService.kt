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
import slingshot.diagnostics.DiagnosticException
import slingshot.diagnostics.DiagnosticProvider
import slingshot.diagnostics.VerilatorDiagnostics
import slingshot.indexing.IndexManager
import slingshot.parsing.ParseUtils
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
        // TODO need to lock the index so its usable across threads without race conditions - use
        //  concurrent hash map
        executor.submit { indexManager.insert(path, document) }

        // now that we know the document is definitely in the index, run diagnostics asynchronously in the
        // thread pool
        executor.submit {
            try {
                val diagnostics = diagnostics.diagnose(path, document)
                client?.publishDiagnostics(PublishDiagnosticsParams(path.toUri().toString(), diagnostics))
            } catch (e: DiagnosticException) {
                Logger.warn("Diagnostics failed for $path:")
                Logger.warn(e)
            }
        }
    }

    override fun completion(position: CompletionParams): CompletableFuture<Either<MutableList<CompletionItem>, CompletionList>> {
        // NOTE: POSITIONS ARE ZERO INDEXED
        Logger.debug("Completion request for ${position.position.line}:${position.position.character} in ${position.textDocument.uri}")

        return CompletableFuture.supplyAsync {
            val path = URI(position.textDocument.uri).toPath()
            val entry = indexManager.retrieve(path) ?: run {
                Logger.warn("Document $path not in index, cannot run completion!")
                return@supplyAsync EMPTY_COMPLETION
            }

            // firstly, we should not return any completion if we are in a comment
            if (ParseUtils.isInAnyComment(entry.contents, position.position.line, position.position.character)) {
                Logger.debug("In comment, will not complete")
                return@supplyAsync EMPTY_COMPLETION
            }

            // if we're not in a comment, then parse the text document to produce a tree, if we don't already
            // have one
            if (entry.tree == null) {
                Logger.debug("Parsing document $path")
                try {
                    entry.tree = completion.parseDocument(entry.contents)
                } catch (e: CompletionException) {
                    Logger.warn("Completion failed for $path:")
                    Logger.warn(e)
                    return@supplyAsync EMPTY_COMPLETION
                }
            }

            val item = CompletionItem("ctr").apply {
                kind = CompletionItemKind.Text
            }
            return@supplyAsync Either.forLeft(mutableListOf(item))
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

    companion object {
        private val EMPTY_COMPLETION = Either.forLeft<MutableList<CompletionItem>, CompletionList>(mutableListOf())
    }
}