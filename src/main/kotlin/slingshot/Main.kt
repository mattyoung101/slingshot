/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot

import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.tree.ParseTreeWalker
import org.eclipse.lsp4j.launch.LSPLauncher
import org.tinylog.kotlin.Logger
import slingshot.completion.DocumentVisitor
import slingshot.completion.LogErrorListener
import slingshot.parser.SystemVerilogLexer
import slingshot.parser.SystemVerilogParser
import slingshot.utils.TreeUtils
import java.io.File

/** Slingshot LSP version */
const val VERSION = "0.1.0"

fun main(args: Array<String>) {
    System.setProperty("tinylog.configuration", "tinylog.properties")
    Logger.info("Slingshot LSP v${VERSION} - Copyright (c) 2023 Matt Young. Mozilla Public License v2.0.")

    val document = File("/home/matt/workspace/waveform/rtl/clk_divider.sv").readText()

    for (i in 0..2) {
        val begin = System.nanoTime()

        val lexer = SystemVerilogLexer(CharStreams.fromString(document))
        lexer.removeErrorListeners()
        lexer.addErrorListener(LogErrorListener)
        val tokens = CommonTokenStream(lexer)

        val parser = SystemVerilogParser(tokens)
        parser.removeErrorListeners()
        parser.addErrorListener(LogErrorListener)

        val tree = parser.source_text()
        val walker = DocumentVisitor()
        ParseTreeWalker.DEFAULT.walk(walker, tree)
        walker.document.finishModule()

//        val ruleNames = SystemVerilogParser.ruleNames.toList()
//        Logger.debug("\n${TreeUtils.toPrettyTree(tree, ruleNames)}")

        Logger.info("Parse took ${(System.nanoTime() - begin) / 1e+6} ms\n")
    }

//    val server = SlingshotServer()
//    val launcher = LSPLauncher.createServerLauncher(server, System.`in`, System.out)
//    launcher.startListening()
}