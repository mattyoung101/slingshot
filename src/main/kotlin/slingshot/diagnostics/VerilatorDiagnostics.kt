/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.diagnostics

import org.eclipse.lsp4j.Diagnostic
import org.eclipse.lsp4j.DiagnosticSeverity
import org.eclipse.lsp4j.Position
import org.eclipse.lsp4j.Range
import org.tinylog.kotlin.Logger
import java.nio.file.Path
import java.util.concurrent.TimeUnit

/**
 * Slingshot diagnostics interface to Verilator. Although Verilator can only read files "written to disk",
 * this file uses a hack of telling Verilator to read from /dev/stdin, meaning it's completely in-memory to
 * reduce disk thrashing :)
 *
 * NOTE: This means it will not work on Windows. If any Windows users appear and complain, we should (maybe)
 * write a workaround for them using a temp file.
 */
class VerilatorDiagnostics : DiagnosticProvider {
    override fun diagnose(path: Path, document: String): List<Diagnostic> {
        val process = ProcessBuilder(VERILATOR_ARGS)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectInput(ProcessBuilder.Redirect.PIPE)
            .directory(path.parent?.toFile() ?: throw DiagnosticException("Path $path has no parent"))
            .start()
        val diagnostics = mutableListOf<Diagnostic>()

        // send the document to verilator, which is listening to /dev/stdin. this is great because it means
        // we don't have to create a temp file, everything is in memory.
        // also send EOF by closing it as well.
        process.outputWriter().use { it.write(document) }
        if (!process.waitFor(5, TimeUnit.SECONDS)) {
            process.destroyForcibly()
            throw DiagnosticException("Verilator timed out")
        }

        val stderr = process.errorReader().use { it.readText() }
        val stdout = process.inputReader().use { it.readText() }
//        Logger.debug("Verilator stdout:\n$stdout\nVerilator stderr:\n$stderr")

        if (process.exitValue() != 0) {
            Logger.trace("Verilator exited with error status: ${process.exitValue()}")

            if (!("Error" in stderr || "Warning" in stderr)) {
                throw DiagnosticException("Verilator did not return any warning/error " +
                 "messages\n$stdout\n$stderr")
            }
        }

        // parse verilator output with regex, unfortunately verilator won't give us machine readable output
        // so we have to do it ourselves
        for (capture in VERILATOR_REGEX.findAll(stderr)) {
            val msgType = capture.groupValues[1]
            val fileName = capture.groupValues[2]
            val line = capture.groupValues[3].toIntOrNull() ?: throw DiagnosticException(
                "Failed to parse line number in document $path\n$stderr"
            )
            val pos = capture.groupValues[4].toIntOrNull() ?: throw DiagnosticException(
                "Failed to parse line pos in document $path\n$stderr"
            )
            val msg = capture.groupValues[5]

            // Sometimes Verilator returns diagnostics for a file we are not in, so ignore them
            if ("/dev/stdin" !in fileName) {
                Logger.trace("Verilator diagnostic applies to unrelated file $fileName - skipping")
                continue
            }

            // For now, we will just report to the user that the error is the entire line
            val endPos = document.lines().getOrNull(line - 1)?.length ?: throw DiagnosticException(
                "Failed to get length of line $line in document $path\n$stderr"
            )
            Logger.trace("msg type: $msgType, line: $line, pos: $pos, msg: $msg")

            val range = Range(
                Position(line - 1, pos - 1),
                Position(line - 1, endPos)
            )
            val diagnostic = Diagnostic(
                range,
                msg,
                if (msgType == "Error") DiagnosticSeverity.Error else DiagnosticSeverity.Warning,
                "verilator"
            )
            diagnostics.add(diagnostic)
        }

        return diagnostics
    }

    companion object {
        private val VERILATOR_ARGS = listOf(
            "verilator", "--lint-only", "-Wall", "-Wno-DECLFILENAME", "/dev/stdin"
        )

        // Verilator warning/error matcher regex
        // This was developed on regex101 and also accounts for the fact that Verilator can split warnings
        // across 2 lines
        // Useful information is in capture groups
        private val VERILATOR_REGEX = Regex("%(Warning|Error).*: (.*):([0-9]+):([0-9]+): (.*)(.*:.*)?")
    }
}