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
import slingshot.completion.ANTLRCompletion
import java.io.File

/** Slingshot LSP version */
const val SLINGSHOT_VERSION = "0.1.0"

fun main(args: Array<String>) {
    System.setProperty("tinylog.configuration", "tinylog.properties")
    Logger.info("Slingshot LSP v${SLINGSHOT_VERSION} - Copyright (c) 2023 Matt Young. Mozilla Public License v2.0.")

    val server = SlingshotServer()
    val launcher = LSPLauncher.createServerLauncher(server, System.`in`, System.out)
    Logger.info("Booting server")
    launcher.startListening()
}
