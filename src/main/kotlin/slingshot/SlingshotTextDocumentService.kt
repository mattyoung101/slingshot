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
import slingshot.diagnostics.DiagnosticUtils
import slingshot.diagnostics.VerilatorDiagnostics
import slingshot.indexing.IndexManager
import slingshot.parsing.CompletionTypes
import slingshot.parsing.ParseUtils
import java.net.URI
import java.nio.file.Path
import java.util.concurrent.CompletableFuture
import java.util.concurrent.ForkJoinPool
import kotlin.io.path.exists
import kotlin.io.path.readText
import kotlin.io.path.toPath

class SlingshotTextDocumentService(var config: SlingshotConfig? = null) : TextDocumentService, LanguageClientAware {
    private val indexManager = IndexManager()
    private val completion: CompletionProvider = ANTLRCompletion()
    private val diagnostics: DiagnosticProvider = VerilatorDiagnostics()
    private var client: LanguageClient? = null
    private val executor = ForkJoinPool.commonPool()

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
            var result: CompletionResult? = null
            if (entry.tree == null) {
                Logger.debug("Parsing document $path")
                try {
                    // we need to both generate our abstract "document tree", so we know what things to return
                    // to the user, AND completion information so we know _what_ from the document tree to
                    // recommend.
                    // this function returns both those things
                    result = completion.parseDocument(entry.contents, position.position.line,
                        position.position.character)

                    // this is stored for the sake of full project indexing and serialisation in the future
                    // if we don't end up doing that, it can probably be removed
                    entry.tree = result.document
                } catch (e: CompletionException) {
                    Logger.warn("Completion failed for $path:")
                    Logger.warn(e)
                    return@supplyAsync EMPTY_COMPLETION
                }
            } else {
                // TODO consider how often this happens and maybe just re-insert every time?
                Logger.debug("Document $path already has parse tree and completion")
            }

            val completion = result!!
            if (completion.recommendations.size == 1 && completion.recommendations[0] == CompletionTypes.None) {
                Logger.warn("No completion recommendations available for $path, cannot run completion")
                return@supplyAsync EMPTY_COMPLETION
            }

            Logger.info("Recommendations: ${completion.recommendations}")

            // the CompletionSelector uses the extracted SvDocument, so we understand the document, and
            // completion recommendations so we know what types of things to send to the user. it then
            // generates actual CompletionItem instances to return to the LSP
            val generator = CompletionGenerator(completion)
            return@supplyAsync Either.forLeft(generator.generate().toMutableList())
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

    private fun parseAndIndex(path: Path) {
        Logger.debug("Parsing and indexing file: $path")

        // we'll just read the file contents from disk, assume user is not editing it rn
        val text = path.readText()
        // we're required to specify a position in the document for the "cursor", but we are not
        // editing this document - so let's just say 0,0. we only care about the SvDocument anyway.
        val result = completion.parseDocument(text, 0, 0)

        // insert the document into the index
        // TODO make a method to insert the document and tree at the same time
        indexManager.insert(path, text)
        indexManager.retrieve(path)!!.tree = result.document
    }

    /**
     * Must be called by [SlingshotServer] after it has been connected to an LSP client, determined
     * the project base directory and loaded the config if possible.
     */
    fun onPostInit(baseDir: Path, config: SlingshotConfig?) {
        Logger.debug("Running onPostInit with baseDir: $baseDir, config: $config")

        // resolve include dirs against base directory
        if (config != null) {
            val includeDirs = mutableListOf<Path>()

            for (dir in config.includeDirs) {
                val includeDir = baseDir.resolve(dir)
                if (!includeDir.exists()) {
                    Logger.warn("Specified include dir '$dir' does not exist! Resolved " +
                     "to '$includeDir' against base dir $baseDir")
                     continue
                }
                includeDirs.add(includeDir)
            }
            Logger.info("Resolved include dirs: $includeDirs")

            // forward to diagnostics
            diagnostics.updateIncludeDirs(includeDirs)

            // find verilog files in the resolved include dirs, parse and index them
            Logger.info("Discovering and indexing SV documents in include dirs")
            for (file in DiagnosticUtils.walkIncludeDirs(includeDirs)) {
                parseAndIndex(file)
            }

            // flush index
            indexManager.flush(baseDir)
        } else {
            Logger.warn("Config is null, can't resolve include dirs!")
        }
    }

    companion object {
        /** An empty completion response for [completion] */
        private val EMPTY_COMPLETION = Either.forLeft<MutableList<CompletionItem>, CompletionList>(mutableListOf())
    }
}
