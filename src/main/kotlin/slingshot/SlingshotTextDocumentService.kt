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
import slingshot.completion.*
import slingshot.config.SlingshotConfig
import slingshot.diagnostics.DiagnosticException
import slingshot.diagnostics.DiagnosticProvider
import slingshot.diagnostics.VerilatorDiagnostics
import slingshot.indexing.IndexManager
import slingshot.parsing.CompletionTypes
import slingshot.parsing.ParseUtils
import java.net.URI
import java.nio.file.Path
import java.util.concurrent.CompletableFuture
import java.util.concurrent.ForkJoinPool
import kotlin.io.path.toPath

class SlingshotTextDocumentService(var config: SlingshotConfig? = null) : TextDocumentService, LanguageClientAware {
    private val indexManager = IndexManager()
    private val completion: CompletionProvider = ANTLRCompletion()
    private val diagnostics: DiagnosticProvider = VerilatorDiagnostics()
    private var client: LanguageClient? = null
    private val executor = ForkJoinPool.commonPool()

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

            // also check if we are in double quotes
            if (ParseUtils.isInDoubleQuotes(entry.contents, position.position.line, position.position.character)) {
                Logger.debug("In double quotes, will not complete")
                return@supplyAsync EMPTY_COMPLETION
            }

            // parse the text document to produce a tree, if we don't already have one on file
            if (entry.tree == null || entry.completion == null) {
                Logger.debug("Parsing document $path")
                try {
                    // we need to both generate our abstract "document tree", so we know what things to return
                    // to the user, AND completion information so we know _what_ from the document tree to
                    // recommend.
                    // this function returns both those things
                    val result = completion.parseDocument(entry.contents, position.position.line,
                        position.position.character)

                    // this is stored for the sake of full project indexing and serialisation in the future
                    // if we don't end up doing that, it can probably be removed
                    entry.tree = result.document
                    entry.completion = result
                } catch (e: CompletionException) {
                    Logger.warn("Completion failed for $path:")
                    Logger.warn(e)
                    return@supplyAsync EMPTY_COMPLETION
                }
            } else {
                // TODO consider how often this happens and maybe just re-insert every time?
                Logger.debug("Document $path already has parse tree and completion")
            }

            val completion = entry.completion!!
            if (completion.recommendations.size == 1 && completion.recommendations[0] == CompletionTypes.None) {
                Logger.warn("No completion recommendations available for $path, cannot run completion")
                return@supplyAsync EMPTY_COMPLETION
            }

            Logger.warn("Recommendations: ${completion.recommendations}")

            // the CompletionSelector uses the extracted SvDocument, so we understand the document, and
            // completion recommendations so we know what types of things to send to the user. it then
            // generates actual CompletionItem instances to return to the LSP
            val selector = CompletionGenerator(completion)
            return@supplyAsync Either.forLeft(selector.generate().toMutableList())
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
        Logger.debug("Common thread pool uses ${executor.parallelism} threads")
        this.client = client
    }

    companion object {
        /** An empty completion response for [completion] */
        private val EMPTY_COMPLETION = Either.forLeft<MutableList<CompletionItem>, CompletionList>(mutableListOf())
    }
}
