/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.tinylog.kotlin.Logger
import slingshot.parser.SystemVerilogPreParser
import slingshot.parser.SystemVerilogPreParserBaseListener

/**
 * This visitor visits the ANTLR tree for the SystemVerilog pre-processor file and adds information to an
 * existing [SvDocument].
 */
class SvPreParseTreeVisitor(private val document: SvDocument) : SystemVerilogPreParserBaseListener() {
    override fun enterText_macro_definition(ctx: SystemVerilogPreParser.Text_macro_definitionContext) {
        document.addMacro("`" + ctx.macro_name().MACRO_NAME().text, ctx.macro_text().text)
    }
}