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
import slingshot.parsing.TokenType
import java.net.URI
import java.nio.file.Path
import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executors
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
     * Called whenever the text document was changed. This function is assumed to _not_ be called from a
     * completable future, hence why the executor is used, so we don't block stuff running elsewhere.
     */
    private fun changed(path: Path, document: String) {
        // store document in index when it's changed
        executor.submit { indexManager.insert(path, document) }

        // at the same time, we run diagnostics asynchronously in the thread pool
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
        return CompletableFuture.supplyAsync {
            val path = URI(position.textDocument.uri).toPath()
            Logger.debug("Completion request for ${position.position.line}:${position.position.character} in $path")

            val entry = indexManager.retrieve(path) ?: run {
                Logger.warn("Document $path not in index, cannot run completion!")
                return@supplyAsync EMPTY_COMPLETION
            }

            // firstly, we should not return any completion if we are in a comment
            if (ParseUtils.isInAnyComment(entry.contents, position.position.line, position.position.character)) {
                Logger.debug("In comment, will not complete")
                return@supplyAsync EMPTY_COMPLETION
            }

            // parse the text document to produce a tree, if we don't already have one on file
            var tokenType = TokenType.Unknown
            if (entry.tree == null) {
                Logger.debug("Parsing document $path")
                try {
                    // this determines both the abstract parse tree and the active token
                    val (parseTree, token) = completion.parseDocument(entry.contents, position.position.line,
                        position.position.character)

                    // store parse tree back into the index, and the token type locally
                    entry.tree = parseTree
                    tokenType = token
                } catch (e: CompletionException) {
                    Logger.warn("Completion failed for $path:")
                    Logger.warn(e)
                    return@supplyAsync EMPTY_COMPLETION
                }
            }

            if (tokenType == TokenType.Unknown) {
                Logger.warn("Unable to determine active token in document $path, cannot run completion")
                return@supplyAsync EMPTY_COMPLETION
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

    override fun didClose(params: DidCloseTextDocumentParams) {}

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
        /** An empty completion response for [completion] */
        private val EMPTY_COMPLETION = Either.forLeft<MutableList<CompletionItem>, CompletionList>(mutableListOf())
    }
}
