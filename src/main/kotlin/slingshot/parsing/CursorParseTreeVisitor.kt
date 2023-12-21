/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.antlr.v4.runtime.ParserRuleContext
import org.eclipse.lsp4j.Position
import org.tinylog.kotlin.Logger
import slingshot.parser.SystemVerilogParser
import slingshot.parser.SystemVerilogParserBaseListener

/**
 * This visitor visits the ANTLR parse tree to find what [TokenType] the user's cursor is on, and optionally
 * also what the name of the module the user is editing in.
 */
class CursorParseTreeVisitor(private val cursor: Position) : SystemVerilogParserBaseListener() {
    /** completion tokens we should recommend to the user */
    var tokenTypes = mutableListOf(CompletionTypes.None)
    /** name of the module the cursor is in */
    var moduleName: String? = null

    // note: we don't have to check depth because we will naturally keep updating tokenType as we go deeper
    // and we check to make sure the cursor is _inside_ the object before updating it

    /**
     * Starts a new completion recommendation by removing the current one iff the cursor is contained by the
     * parse item. If it is, then the code in the block will be executed
     */
    private fun start(ctx: ParserRuleContext, block: () -> Unit) {
        val start = ctx.start.toPosition()
        val end = ctx.stop.toPosition()
        Logger.debug("Rule ${ctx.javaClass.simpleName}, start: ${start.toShortString()}, end: ${end.toShortString()}, cursor: ${cursor.toShortString()}")

        if (cursor.containedIn(start, end)) {
            Logger.debug("    Start new completion: ${ctx.javaClass.simpleName}")
            tokenTypes.clear()
            block()
        } else {
            Logger.debug("    Not contained by cursor.")
        }
    }

    /** Recommends a completion item. Must call [start] first. */
    private fun recommend(token: CompletionTypes) {
        tokenTypes.add(token)
    }

    /** Recommends general variable types */
    private fun recommendVariableTypes() {
        recommend(CompletionTypes.Macro)
        recommend(CompletionTypes.EnumValue)
        recommend(CompletionTypes.VariableSameModule)
        recommend(CompletionTypes.PortSameModule)
    }

    override fun enterModule_declaration(ctx: SystemVerilogParser.Module_declarationContext) {
        // generate module name, if cursor is in module
        val start = ctx.start.toPosition()
        val end = ctx.stop.toPosition()
        if (cursor.containedIn(start, end)) {
            moduleName = ctx.module_header()?.module_identifier()?.text
        }

        // in this state, we can basically recommend to complete all top level items
        start(ctx) {
            recommend(CompletionTypes.Module)
            recommend(CompletionTypes.Enum)
            recommend(CompletionTypes.Macro)
            recommend(CompletionTypes.Logic)
            recommend(CompletionTypes.Always)
            recommend(CompletionTypes.SystemTask)
        }
    }

    // in event expressions like @(posedge clk) we should suggest variable names
    override fun enterEvent_expression(ctx: SystemVerilogParser.Event_expressionContext) {
        start(ctx) {
            recommendVariableTypes()
            recommend(CompletionTypes.Edge)
        }
    }

    // in event expressions like @(posedge clk) we should suggest variable names
    // FIXME we should _not_ have to recommend on enterEvent_control, but the range on enterEvent_expression is wrong
    override fun enterEvent_control(ctx: SystemVerilogParser.Event_controlContext) {
        start(ctx) {
            recommendVariableTypes()
            recommend(CompletionTypes.Edge)
        }
    }

    // in an if statement, recommend a variable
    override fun enterCond_predicate(ctx: SystemVerilogParser.Cond_predicateContext) {
        start(ctx) {
            recommendVariableTypes()
        }
    }

    // in normal code (sequential block) recommend a variable or system task
    override fun enterSeq_block(ctx: SystemVerilogParser.Seq_blockContext) {
        start(ctx) {
            recommendVariableTypes()
            recommend(CompletionTypes.SystemTask)
        }
    }

    // this seems to occur a lot when parse fails, but it's typically a top level declaration so we should
    // recommend data types and stuff
    override fun enterModule_identifier(ctx: SystemVerilogParser.Module_identifierContext) {
        start(ctx) {
            recommend(CompletionTypes.Module)
            recommend(CompletionTypes.Enum)
            recommend(CompletionTypes.Macro)
            recommend(CompletionTypes.Logic)
            recommend(CompletionTypes.SystemTask)
        }
    }

    // in ranges like [1:5] we want to explicitly _not_ complete
    override fun enterConstant_range(ctx: SystemVerilogParser.Constant_rangeContext) {
        start(ctx) {
            recommend(CompletionTypes.None)
        }
    }
}