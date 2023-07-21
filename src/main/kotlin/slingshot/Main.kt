package slingshot

import org.antlr.v4.runtime.*
import org.eclipse.lsp4j.launch.LSPLauncher
import org.tinylog.kotlin.Logger
import slingshot.parser.SystemVerilogLexer
import slingshot.parser.SystemVerilogParser

/** Slingshot LSP version */
const val VERSION = "0.1.0"

object SlingErrorListener : BaseErrorListener() {
    override fun syntaxError(
        recognizer: Recognizer<*, *>?,
        offendingSymbol: Any?,
        line: Int,
        charPositionInLine: Int,
        msg: String?,
        e: RecognitionException?
    ) {
        Logger.error("Syntax error at $line:$charPositionInLine: $msg")
    }
}

fun main(args: Array<String>) {
    System.setProperty("tinylog.configuration", "tinylog.properties")
    Logger.info("Slingshot LSP v${VERSION} - Copyright (c) 2023 Matt Young. Mozilla Public License v2.0.")

    val document = """
        `timescale 1ns/1ns
        `default_nettype none
        `include "defines.vh"

        // This module implements a 6 MHz -> 1 MHz clock divider
        module clk_div_6to1 (
            // Input 6 MHz clock
            input logic i_clk,

            // Reset line
            input logic i_rst,

            input logic frog,

            // Output 1 MHz clock
            output logic o_clk
        );
            // Counter
            logic[2:0] ctr;

            always_ff @(posedge i_clk) begin
                if (i_rst) begin
                    ctr <= 0;
                end else begin
                    // every 6 cycles, give a rising edge to i_clk, this should mean i_clk is 1 MHz as required
                    if (ctr == 6) begin
                        o_clk <= 1;
                        // reset counter !important!
                        ctr <= 0;
                    end else begin
                        o_clk <= 0;
                        // increment timer
                        ctr <= ctr + 1;
                    end
                end
            end
        endmodule
    """.trimIndent()

    val begin = System.nanoTime()

    val lexer = SystemVerilogLexer(CharStreams.fromString(document))
    lexer.addErrorListener(SlingErrorListener)
    val tokens = CommonTokenStream(lexer)

    val parser = SystemVerilogParser(tokens)
    parser.addErrorListener(SlingErrorListener)

    val tree = parser.source_text()

    Logger.info("Parse took ${(System.nanoTime() - begin) / 1e+6} ms")

//    val server = SlingshotServer()
//    val launcher = LSPLauncher.createServerLauncher(server, System.`in`, System.out)
//    launcher.startListening()
}