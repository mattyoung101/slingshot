/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.antlr.v4.runtime.Token
import org.eclipse.lsp4j.Position
import org.eclipse.lsp4j.util.Positions
import org.tinylog.kotlin.Logger
import java.lang.IllegalArgumentException

fun Token.toPosition(): Position {
    // ANTLR is 1-indexed for the line, LSP is 0-indexed
    // I don't know why the charPositionInLine isn't zero indexed either?
    return Position(line - 1, charPositionInLine)
}

fun Position.toShortString(): String {
    return "($line, $character)"
}

/**
 * Returns true if this position is contained in [start] to [end], inclusive.
 */
fun Position.containedIn(start: Position, end: Position): Boolean {
    // if it's the exact same position we do actually say it's contained in
    if (start == this && end == this) return true

    // if they're across separate lines we know it definitely contains the cursor
    if (line > start.line && line < end.line) return true

    // otherwise they must be on the same line, and check the line positions as well
    return start.line <= line && end.line >= line && start.character <= character && end.character >= character
}

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
        var isOpening = true
        var opening = -1
        for ((i, lineStr) in document.lines().withIndex()) {
            // quick check for single line block comments
            // FIXME this will not work for: /* comment */ [cursor here]
            if ("/*" in lineStr && "*/" in lineStr && i == line) {
                Logger.trace("Found single line doc comment for line $i")
                return true
            }

            val lookingFor = if (isOpening) "/*" else "*/"
            // continue on if no comment in line
            if (lookingFor !in lineStr) continue

            if (isOpening) {
                Logger.trace("Found opening block comment on line $i")
                opening = i
                isOpening = false
            } else {
                Logger.trace("Found closing block comment on line $i")
                if (line in opening..i) {
                    Logger.trace("Contains line $line")
                    return true
                }
                // no luck, try look for another block comment
                isOpening = true
            }
        }
        return false
    }

    fun isInAnyComment(document: String, line: Int, pos: Int): Boolean {
        return isInLineComment(document, line, pos) || isInBlockComment(document, line)
    }
}