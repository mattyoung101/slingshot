/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot

import org.eclipse.lsp4j.jsonrpc.messages.ResponseError
import org.eclipse.lsp4j.jsonrpc.messages.ResponseErrorCode
import org.eclipse.lsp4j.launch.LSPLauncher
import org.eclipse.lsp4j.services.LanguageClient
import org.tinylog.jul.JulTinylogBridge
import org.tinylog.kotlin.Logger
import kotlin.system.exitProcess

/** Slingshot LSP version */
const val SLINGSHOT_VERSION = "0.1.0"

fun main(args: Array<String>) {
    System.setProperty("tinylog.configuration", "tinylog.properties")
    JulTinylogBridge.activate()
    Logger.info("Slingshot LSP v${SLINGSHOT_VERSION} - Copyright (c) 2023 Matt Young. Mozilla Public License v2.0.")

    Thread.setDefaultUncaughtExceptionHandler { t, e ->
        Logger.error("Uncaught exception in thread $t:\n$e")
        Logger.error(e)
        exitProcess(1)
    }

    val server = SlingshotServer()
    Logger.info("Booting server")

    val launcher = LSPLauncher.Builder<LanguageClient>()
        .setLocalService(server)
        .setRemoteInterface(LanguageClient::class.java)
        .setInput(System.`in`)
        .setOutput(System.out)
        .setExceptionHandler {
            Logger.error("Uncaught exception in LSP callback: $it")
            Logger.error(it)
            return@setExceptionHandler ResponseError(ResponseErrorCode.InternalError, it.javaClass.simpleName, it)
        }
        .create()

    server.connect(launcher.remoteProxy)
    launcher.startListening()
}
