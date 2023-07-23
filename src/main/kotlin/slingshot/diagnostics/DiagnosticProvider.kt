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
import kotlin.jvm.Throws

/**
 * Interface to a piece of software that can perform diagnostics, e.g. Slang, Verilator, etc.
 */
interface DiagnosticProvider {
    /**
    * Provides a set of diagnostics for the given document. Throws [DiagnosticException] if the diagnostic
    * provider failed to generate a set of valid diagnostics.
    */
    @Throws(DiagnosticException::class)
    fun diagnose(path: Path, document: String): List<Diagnostic>
}