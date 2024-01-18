/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.completion

import org.eclipse.lsp4j.CompletionItem
import org.eclipse.lsp4j.CompletionItemKind
import org.eclipse.lsp4j.InsertTextFormat
import org.tinylog.kotlin.Logger
import slingshot.indexing.Index
import slingshot.indexing.IndexManager
import slingshot.parsing.CompletionTypes
import slingshot.parsing.SvDocument


/**
 * Using a list of [CompletionTypes] recommendations generated by CursorParseTreeVisitor, and a
 * [SvDocument], generate the actual completion items by analysing the SvDocument instance.
 */
class CompletionGenerator(private val completion: CompletionResult, private val index: IndexManager) {
    private val documents = index.getAllDocuments()

    /** Recommends "variables" (ports, wires, etc) in the current module */
    private fun generateVariableSameModule(): List<CompletionItem> {
        completion.activeModule ?: return listOf()
        return completion.document.getModuleByName(completion.activeModule).variables.map {
            CompletionItem(it.name).apply { kind = CompletionItemKind.Variable }
        }
    }

    /** Recommends only ports in the current module */
    private fun generatePortSameModule(): List<CompletionItem> {
        completion.activeModule ?: return listOf()
        return completion.document.getModuleByName(completion.activeModule).ports.map {
            CompletionItem(it.name).apply { kind = CompletionItemKind.Field }
        }
    }

    /** Recommends the name of a module in the current document */
    private fun generateModule(): List<CompletionItem> {
        return documents.flatMap { it.modules }.map {
            CompletionItem(it.name).apply { kind = CompletionItemKind.Module }
        }
    }

    /** Recommends "posedge" or "negedge" */
    private fun generateEdge(): List<CompletionItem> {
        val posedge = CompletionItem("posedge").apply { kind = CompletionItemKind.Keyword }
        val negedge = CompletionItem("negedge").apply { kind = CompletionItemKind.Keyword }
        return listOf(posedge, negedge)
    }

    /** Recommends "logic" */
    private fun generateLogic(): List<CompletionItem> {
        val logic = CompletionItem("logic").apply { kind = CompletionItemKind.Keyword }
        // maybe add wire, reg if people ask for verilog support
        return listOf(logic)
    }

    /** Recommends snippets for "always_comb", "always_ff" and "always_latch" */
    private fun generateAlways(): List<CompletionItem> {
        val alwaysComb = CompletionItem("always_comb").apply {
            insertText = "always_comb begin\n$0\nend"
            kind = CompletionItemKind.Snippet
            insertTextFormat = InsertTextFormat.Snippet
        }
        val alwaysLatch = CompletionItem("always_latch").apply {
            insertText = "always_latch begin\n$0\nend"
            kind = CompletionItemKind.Snippet
            insertTextFormat = InsertTextFormat.Snippet

        }
        val alwaysFf = CompletionItem("always_ff").apply {
            insertText = "always_ff @($0) begin\n\nend"
            kind = CompletionItemKind.Snippet
            insertTextFormat = InsertTextFormat.Snippet
        }

        return listOf(alwaysComb, alwaysFf, alwaysLatch)
    }

    private fun generateCase(): List<CompletionItem> {
        val out = mutableListOf<CompletionItem>()
        for (case in listOf("case", "casex", "casez", "unique case")) {
            val completion = CompletionItem(case).apply {
                insertText = "$case ($0) begin\n    default: /* todo */ \nendcase"
                kind = CompletionItemKind.Snippet
                insertTextFormat = InsertTextFormat.Snippet
            }
            out.add(completion)
        }
        return out
    }

    /** Recommends the name of an enum */
    private fun generateEnum(): List<CompletionItem> {
        return documents.flatMap { it.enums }.map {
            CompletionItem(it.name).apply { kind = CompletionItemKind.Enum }
        }
    }

    /** Recommends the contents of an enum */
    private fun generateEnumValue(): List<CompletionItem> {
        return documents.flatMap { it.enums }.flatMap { it.enumValues }.map {
            CompletionItem(it.name).apply { kind = CompletionItemKind.EnumMember }
        }
    }

    /** Recommends a macro */
    private fun generateMacros(): List<CompletionItem> {
        return documents.flatMap { it.macros }.map {
            CompletionItem(it.name).apply { kind = CompletionItemKind.Variable }
        }
    }

    /** Recommends SV system tasks */
    private fun generateSystemTasks(): List<CompletionItem> {
        return SYSTEM_TASKS.map { CompletionItem("$$it").apply { kind = CompletionItemKind.Function } }
    }

    /**
     * Generates completion items to return to the user based on the provided [completion]
     */
    fun generate(): List<CompletionItem> {
        val out = mutableListOf<CompletionItem>()

        for (rec in completion.recommendations) {
            when (rec) {
                CompletionTypes.VariableSameModule -> out.addAll(generateVariableSameModule())
                CompletionTypes.PortSameModule -> out.addAll(generatePortSameModule())
                CompletionTypes.Edge -> out.addAll(generateEdge())
                CompletionTypes.Logic -> out.addAll(generateLogic())
                CompletionTypes.Module -> out.addAll(generateModule())
                CompletionTypes.Enum -> out.addAll(generateEnum())
                CompletionTypes.EnumValue -> out.addAll(generateEnumValue())
                CompletionTypes.Always -> out.addAll(generateAlways())
                CompletionTypes.Macro -> out.addAll(generateMacros())
                CompletionTypes.SystemTask -> out.addAll(generateSystemTasks())
                CompletionTypes.Case -> out.addAll(generateCase())
                else -> Logger.warn("Unhandled recommendation type: $rec")
            }
        }

        return out
    }

    companion object {
        /** Partial list of commonly used SystemVerilog system tasks */
        private val SYSTEM_TASKS = setOf(
            "display",
            "monitor",
            "write",
            "strobe",
            "error",
            "fatal",
            "info",
            "warning",
            "clog2",
            "finish",
            "stop",
            "fopen",
            "fscanf",
            "fwrite",
            "fgets",
            "readmemb",
            "readmemh",
            "write",
            "floor",
            "ceil",
            "countones",
            "countbits",
            "time",
            "signed",
            "unsigned"

            // These pollute the completion list quite a bit, and I can't imagine they are seriously commonly
            // used, so I'm not including them for now.
//            "asin",
//            "acos",
//            "atan",
//            "atan2",
//            "sin",
//            "cos",
//            "tan",
//            "ln",
//            "log10",
//            "exp",
//            "sqrt",
//            "hypot",
//            "pow",

        )
    }
}