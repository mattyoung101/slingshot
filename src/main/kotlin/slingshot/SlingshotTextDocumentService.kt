/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot

import org.eclipse.lsp4j.DidChangeTextDocumentParams
import org.eclipse.lsp4j.DidCloseTextDocumentParams
import org.eclipse.lsp4j.DidOpenTextDocumentParams
import org.eclipse.lsp4j.DidSaveTextDocumentParams
import org.eclipse.lsp4j.services.TextDocumentService
import org.tinylog.kotlin.Logger
import slingshot.completion.ANTLRCompletion
import slingshot.completion.CompletionException
import slingshot.completion.CompletionProvider
import slingshot.indexing.IndexManager
import java.net.URI
import java.nio.file.Path
import kotlin.io.path.toPath

class SlingshotTextDocumentService : TextDocumentService {
    private val indexManager = IndexManager()
    private val completion: CompletionProvider = ANTLRCompletion()

    /** Called when the LSP is shutting down. May be called more than once. */
    fun onShutdown() {
        Logger.info("LSP shutting down")
        // nothing at the moment, but we would flush the index
    }

    /**
     * Called whenever the text document was changed
     */
    private fun changed(path: Path, document: String) {
        // run diagnostics

        // run completion
        try {
            val tree = completion.parseDocument(document)
            indexManager.insert(path, document, tree)
        } catch (ex: CompletionException) {
            Logger.warn("Failed to complete document $path:")
            Logger.warn(ex)
            Logger.debug("Failed completion document text:\n$document")
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
}