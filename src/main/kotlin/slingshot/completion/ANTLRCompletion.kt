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
import slingshot.parser.SystemVerilogParser.Source_textContext
import slingshot.parser.SystemVerilogPreParser
import slingshot.parsing.*
import slingshot.utils.TreeUtils

/**
 * Completion provider that uses an ANTLR 4 grammar to generate an [SvDocument].
 * This is the best completion provider for Slingshot.
 */
class ANTLRCompletion : CompletionProvider {
    private fun parseDocument(document: String): Pair<SvDocument, Source_textContext>  {
        Logger.debug("Parsing SV document")

        // remove problematic macros
        val lexer = SystemVerilogLexer(CharStreams.fromString(ParseUtils.stripProblematicMacros(document)))
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

        return Pair(documentVisitor.document, tree)
    }

    private fun updateWithPreprocessor(document: String, svDocument: SvDocument) {
        Logger.debug("Parsing SV document with pre-processor")
        // FIXME shouldn't we share macroLexer from this function and lexer from parseDocument?
        val macroLexer = SystemVerilogLexer(CharStreams.fromString(document))

        // change to directive channel: https://stackoverflow.com/a/18636296/5007892
        val directiveIdx = macroLexer.channelNames.indexOf("DIRECTIVES")
//        Logger.debug("Lexer channels: ${macroLexer.channelNames.joinToString(" ")}")
//        Logger.debug("Directive index: $directiveIdx")
        macroLexer.channel = directiveIdx
        val tokens = CommonTokenStream(macroLexer, directiveIdx)
//        Logger.debug("Tokens: ${tokens.tokens.joinToString(", ") { it.text }}")

        val macroParser = SystemVerilogPreParser(tokens)
        macroParser.removeErrorListeners()
        macroParser.addErrorListener(LogErrorListener)

        val macroTree = macroParser.source_text()

        // update the SvDocument with macros
        val preprocVisitor = SvPreParseTreeVisitor(svDocument)
        ParseTreeWalker.DEFAULT.walk(preprocVisitor, macroTree)
//        Logger.debug(TreeUtils.toPrettyTree(macroTree, SystemVerilogPreParser.ruleNames.toMutableList()))
    }

    override fun parseDocument(document: String, line: Int, pos: Int): CompletionResult {
        val begin = System.nanoTime()

        // first pass: parse the document
        val (svDocument, tree) = parseDocument(document)

        // second pass: parse macros
        updateWithPreprocessor(document, svDocument)

        // figure out what to recommend to the user
        val cursorVisitor = CursorParseTreeVisitor(Position(line, pos))
        ParseTreeWalker.DEFAULT.walk(cursorVisitor, tree)

        // consider running walkers in parallel if performance becomes an issue, profile first though!
        // we are still like 2 ms so I expect we're completely fine for the forseeable future
        Logger.debug("Cursor visitor will recommend: ${cursorVisitor.tokenTypes}, inside module: ${cursorVisitor.moduleName}")
        Logger.debug("Parse took ${(System.nanoTime() - begin) / 1e+6} ms")

        return CompletionResult(svDocument, cursorVisitor.tokenTypes, cursorVisitor.moduleName)
    }
}
