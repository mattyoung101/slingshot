/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.tinylog.kotlin.Logger
import slingshot.parser.SystemVerilogParser
import slingshot.parser.SystemVerilogParserBaseListener

/**
 * This visitor visits the ANTLR tree for a SystemVerilog file and turns it into a SvDocument
 * which will be used in completion and indexing.
 */
class SvParseTreeVisitor : SystemVerilogParserBaseListener() {
   val document = SvDocument()

    override fun enterModule_declaration(ctx: SystemVerilogParser.Module_declarationContext?) {
        val name = ctx?.module_header()?.module_identifier()?.text ?:
            return Logger.error("Unable to determine name for module declaration: $ctx")
        document.newModule(name)
    }

    override fun exitModule_declaration(ctx: SystemVerilogParser.Module_declarationContext?) {
        document.finishModule()
    }

    override fun enterAnsi_port_declaration(ctx: SystemVerilogParser.Ansi_port_declarationContext?) {
        val name = ctx?.port_identifier()?.identifier()?.text ?: return
            Logger.error("Unable to determine name for port declaration: $ctx")
        document.addPort(SvToken(name, TokenType.Port))
    }

    override fun enterVariable_decl_assignment(ctx: SystemVerilogParser.Variable_decl_assignmentContext?) {
        val name = ctx?.variable_identifier()?.identifier()?.text ?: return
            Logger.error("Unable to determine name for variable declaration: $ctx")

        val variable = SvToken(name, TokenType.Variable)
        // special case: if this is a variable identifier, make sure we haven't previously recorded it as
        // a port
        // TODO check if this still applies to ANTLR parser
        if (!document.isVarDeclaredAsPort(variable)) {
            document.addVariable(variable)
        } else {
            Logger.debug("    Skipping variable $variable which was already declared as port")
        }
    }
}