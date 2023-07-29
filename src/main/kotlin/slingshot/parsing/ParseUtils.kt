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

object ParseUtils {
    fun isInLineComment(document: String, line: Int, pos: Int): Boolean {
        // return true if the position of "//" in the line is before the current pos
        val idx = document.lines()[line].trim().indexOf("//")
        return if (idx == -1) {
            // no comment in line
            false
        } else {
            idx <= pos
        }
    }

    fun isInBlockComment(document: String, line: Int): Boolean {
        // Logger.debug("Checking if line $line in doc comment")
        var isOpening = true
        var opening = -1
        for ((i, lineStr) in document.lines().withIndex()) {
            // quick check for single line block comments
            if ("/*" in lineStr && "*/" in lineStr && i == line) {
                Logger.debug("Found single line doc comment for line $i")
                return true
            }

            val lookingFor = if (isOpening) "/*" else "*/"
            // no comment in line
            if (lookingFor !in lineStr) continue

            if (isOpening) {
                Logger.debug("Found opening block comment on line $i")
                opening = i
                isOpening = false
            } else {
                Logger.debug("Found closing block comment on line $i")
                if (line in opening..i) {
                    Logger.debug("Contains line $line")
                    return true
                }
                // try look for another block comment
                isOpening = true
            }
        }
        return false
    }

    fun isInAnyComment(document: String, line: Int, pos: Int): Boolean {
        return isInLineComment(document, line, pos) || isInBlockComment(document, line)
    }
}