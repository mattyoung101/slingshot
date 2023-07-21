package slingshot

import org.eclipse.lsp4j.InitializeParams
import org.eclipse.lsp4j.InitializeResult
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
        return CompletableFuture()
    }

    override fun shutdown(): CompletableFuture<Any> {
        return CompletableFuture()
    }

    override fun exit() {
    }

    override fun getTextDocumentService(): TextDocumentService {
        return textDocumentService
    }

    override fun getWorkspaceService(): WorkspaceService {
        return workspaceService
    }
}