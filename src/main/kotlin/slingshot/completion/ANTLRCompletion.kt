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
import org.eclipse.lsp4j.Position
import org.tinylog.kotlin.Logger
import slingshot.parser.SystemVerilogLexer
import slingshot.parser.SystemVerilogParser
import slingshot.parsing.*

/**
 * Completion provider that uses an ANTLR 4 grammar to generate an [SvDocument].
 * This is the best completion provider for Slingshot.
 */
class ANTLRCompletion : CompletionProvider {
    override fun parseDocument(document: String, line: Int, pos: Int): CompletionResult {
        val begin = System.nanoTime()

        val lexer = SystemVerilogLexer(CharStreams.fromString(document))
        lexer.removeErrorListeners()
        lexer.addErrorListener(LogErrorListener)
        val tokens = CommonTokenStream(lexer)

        val parser = SystemVerilogParser(tokens)
        parser.removeErrorListeners()
        parser.addErrorListener(LogErrorListener)

        val tree = parser.source_text()

        // construct the SvDocument
        val documentVisitor = SvParseTreeVisitor()
        ParseTreeWalker.DEFAULT.walk(documentVisitor, tree)
        documentVisitor.document.finishModule()

        // figure out what to recommend to the user
        val cursorVisitor = CursorParseTreeVisitor(Position(line, pos))
        ParseTreeWalker.DEFAULT.walk(cursorVisitor, tree)

        Logger.debug("Cursor visitor: ${cursorVisitor.tokenTypes}, ${cursorVisitor.moduleName}")

       // val ruleNames = SystemVerilogParser.ruleNames.toList()
       // Logger.debug("\n${TreeUtils.toPrettyTree(tree, ruleNames)}")

        Logger.debug("Parse took ${(System.nanoTime() - begin) / 1e+6} ms")

        return CompletionResult(documentVisitor.document, cursorVisitor.tokenTypes, cursorVisitor.moduleName)
    }
}
