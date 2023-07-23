/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.diagnostics

import org.eclipse.lsp4j.Diagnostic
import java.nio.file.Path
import java.util.concurrent.TimeUnit

/** Slingshot diagnostics interface to Verilator */
class VerilatorDiagnostics : DiagnosticProvider {
    override fun diagnose(path: Path, document: String): List<Diagnostic> {
        val process = ProcessBuilder(VERILATOR_ARGS)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectInput(ProcessBuilder.Redirect.PIPE)
            .start()

        // send the document to verilator, which is listening to /dev/stdin
        // also send EOF by closing it as well
        process.outputStream.writer().use { it.write(document) }
        if (!process.waitFor(5, TimeUnit.SECONDS)) {
            process.destroyForcibly()
            throw DiagnosticException("Verilator timed out")
        }

        return listOf()
    }

    companion object {
        val VERILATOR_ARGS = listOf("verilator", "--lint-only", "-Wall", "-Wno-DECLFILENAME", "/dev/stdin");
    }
}