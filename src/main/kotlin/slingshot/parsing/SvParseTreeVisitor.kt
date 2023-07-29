/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.eclipse.lsp4j.Position
import org.tinylog.kotlin.Logger
import slingshot.completion.CompletionException
import slingshot.parser.SystemVerilogParser
import slingshot.parser.SystemVerilogParserBaseListener

/**
 * This visitor visits the ANTLR tree for a SystemVerilog file and turns it into a SvDocument
 * which will be used in completion and indexing.
 */
class SvParseTreeVisitor : SystemVerilogParserBaseListener() {
    val document = SvDocument()

    override fun enterModule_declaration(ctx: SystemVerilogParser.Module_declarationContext?) {
        val name = ctx?.module_header()?.module_identifier()?.text ?: throw CompletionException(
            "Unable to determine name for module declaration: ${ctx?.text}"
        )
        val begin = Position(ctx.start.line, ctx.start.charPositionInLine)
        val end = Position(ctx.stop.line, ctx.stop.charPositionInLine)
        document.newModule(name, begin, end)
    }

    override fun exitModule_declaration(ctx: SystemVerilogParser.Module_declarationContext?) {
        document.finishModule()
    }

    override fun enterAnsi_port_declaration(ctx: SystemVerilogParser.Ansi_port_declarationContext?) {
        val name = ctx?.port_identifier()?.identifier()?.text ?: throw CompletionException(
            "Unable to determine name for port declaration: ${ctx?.text}"
        )
        val begin = Position(ctx.start.line, ctx.start.charPositionInLine)
        val end = Position(ctx.stop.line, ctx.stop.charPositionInLine)
        document.addPort(name, begin, end)
    }

    override fun enterVariable_decl_assignment(ctx: SystemVerilogParser.Variable_decl_assignmentContext?) {
        val name = ctx?.variable_identifier()?.identifier()?.text ?: throw CompletionException(
            "Unable to determine name for variable declaration: ${ctx?.text}"
        )

        // special case: if this is a variable identifier, make sure we haven't previously recorded it as
        // a port
        // TODO check if this still applies to ANTLR parser, we could skip this expensive check
        if (!document.isVarDeclaredAsPort(name)) {
            val begin = Position(ctx.start.line, ctx.start.charPositionInLine)
            val end = Position(ctx.stop.line, ctx.stop.charPositionInLine)
            document.addVariable(name, begin, end)
        } else {
            Logger.debug("    Skipping variable $name which was already declared as port")
        }
    }
}