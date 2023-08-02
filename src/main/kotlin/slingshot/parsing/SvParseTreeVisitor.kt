/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

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
        document.newModule(name)
    }

    override fun exitModule_declaration(ctx: SystemVerilogParser.Module_declarationContext?) {
        document.finishModule()
    }

    override fun enterAnsi_port_declaration(ctx: SystemVerilogParser.Ansi_port_declarationContext?) {
        val name = ctx?.port_identifier()?.identifier()?.text ?: throw CompletionException(
            "Unable to determine name for port declaration: ${ctx?.text}"
        )
        document.addPort(name)
    }

    override fun enterVariable_decl_assignment(ctx: SystemVerilogParser.Variable_decl_assignmentContext?) {
        val name = ctx?.variable_identifier()?.identifier()?.text ?: throw CompletionException(
            "Unable to determine name for variable declaration: ${ctx?.text}"
        )

        document.addVariable(name)
    }

    override fun enterType_declaration(ctx: SystemVerilogParser.Type_declarationContext) {
        if (ctx.data_type()?.ENUM() == null) {
            Logger.debug("Typedef is not enum, skipping")
            return
        }
        val enumName = ctx.type_identifier(0).identifier().text
        document.newEnum(enumName)
    }

    override fun enterEnum_identifier(ctx: SystemVerilogParser.Enum_identifierContext) {
        if (!document.isEnumActive()) return
        document.addEnumValue(ctx.identifier().text)
    }

    override fun exitType_declaration(ctx: SystemVerilogParser.Type_declarationContext?) {
        document.finishEnum()
    }
}