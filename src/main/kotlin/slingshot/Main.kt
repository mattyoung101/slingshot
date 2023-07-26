/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot

import org.eclipse.lsp4j.launch.LSPLauncher
import org.tinylog.kotlin.Logger
import kotlin.system.exitProcess

/** Slingshot LSP version */
const val SLINGSHOT_VERSION = "0.1.0"

fun main(args: Array<String>) {
    System.setProperty("tinylog.configuration", "tinylog.properties")
    Logger.info("Slingshot LSP v${SLINGSHOT_VERSION} - Copyright (c) 2023 Matt Young. Mozilla Public License v2.0.")

    Thread.setDefaultUncaughtExceptionHandler { t, e ->
        Logger.error("Uncaught exception in thread $t:\n$e")
        Logger.error(e)
        exitProcess(1)
    }

    val server = SlingshotServer()
    Logger.info("Booting server")
    val launcher = LSPLauncher.createServerLauncher(server, System.`in`, System.out)
    server.connect(launcher.remoteProxy)
    launcher.startListening()
}
