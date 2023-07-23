/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.completion

import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.tree.ParseTreeWalker
import org.tinylog.kotlin.Logger
import slingshot.parser.SystemVerilogLexer
import slingshot.parser.SystemVerilogParser
import slingshot.parsing.SvDocument

/**
 * Completion provider that uses an ANTLR 4 grammar to generate an [SvDocument].
 * This is the best completion provider for Slingshot.
 */
class ANTLRCompletion : CompletionProvider {
    override fun parseDocument(document: String): SvDocument {
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

        return walker.document
    }
}