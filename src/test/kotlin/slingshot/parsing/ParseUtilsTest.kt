/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.eclipse.lsp4j.Position
import org.junit.jupiter.api.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ParseUtilsTest {
    @Test
    fun testLineComment_Simple() {
        val line1 = "// obvious line comment"
        assertTrue(ParseUtils.isInLineComment(line1, 0, 5))
        assertTrue(ParseUtils.isInAnyComment(line1, 0, 5))
        assertFalse(ParseUtils.isInBlockComment(line1, 0))
    }

    @Test
    fun testLineComment_ExtraneousText() {
        val line2 = "some invalid text // line comment here"
        assertFalse(ParseUtils.isInLineComment(line2, 0, 4))
        assertFalse(ParseUtils.isInAnyComment(line2, 0, 4))
        assertFalse(ParseUtils.isInBlockComment(line2, 0))
    }

    @Test
    fun testLineComment_Whitespace() {
        val line3 = "      \t\t// line comment"
        assertTrue(ParseUtils.isInLineComment(line3, 0, 16))
        assertTrue(ParseUtils.isInAnyComment(line3, 0, 16))
        assertFalse(ParseUtils.isInBlockComment(line3, 0))
    }

    @Test
    fun testNoComment() {
        val line = "line with no comment"
        assertFalse(ParseUtils.isInAnyComment(line, 0, 0))
    }

    @Test
    fun testBlockComment_Simple() {
        val document = """
            /*
             * doc comment
             */
        """.trimIndent()
        assertTrue(ParseUtils.isInBlockComment(document, 1))
        assertTrue(ParseUtils.isInAnyComment(document, 1, 4))
        assertFalse(ParseUtils.isInLineComment(document, 1, 4))
    }

    @Test
    fun testBlockComment_NotInComment() {
        val document = """
            /*
             * doc comment
             */
             extra stuff
             more extra stuff
        """.trimIndent()
        assertFalse(ParseUtils.isInBlockComment(document, 4))
        assertFalse(ParseUtils.isInAnyComment(document, 4, 4))
        assertFalse(ParseUtils.isInLineComment(document, 4, 4))
    }

    @Test
    fun testBlockComment_Inline() {
        val document = "/* doc comment */"
        assertTrue(ParseUtils.isInBlockComment(document, 0))
        assertTrue(ParseUtils.isInAnyComment(document, 0, 5))
    }

    @Test
    fun testPositionContains_NotInBlock() {
        val start = Position(31, 26)
        val end = Position(35, 12)
        val cursor = Position(25, 29)
        assertFalse(cursor.containedIn(start, end))
    }

    @Test
    fun testPositionContains_SameLine() {
        val start = Position(5, 1)
        val end = Position(5, 100)
        val cursor = Position(5, 50)
        assertTrue(cursor.containedIn(start, end))
    }

    @Test
    fun testPositionContains_NotSameLine() {
        val start = Position(5, 10)
        val end = Position(5, 20)
        val cursor = Position(5, 50)
        assertFalse(cursor.containedIn(start, end))
    }

    @Test
    fun testPositionContains_Block() {
        val start = Position(1, 5)
        val end = Position(100, 25)
        val cursor = Position(50, 100)
        assertTrue(cursor.containedIn(start, end))
    }

    @Test
    fun testPositionContains_ZeroWidth() {
        val start = Position(1, 1)
        assertTrue(start.containedIn(start, start))
    }

    @Test
    fun testInDoubleQuotes_Simple() {
        val document = "\$system(\"testing\");"
        assertTrue(ParseUtils.isInDoubleQuotes(document, 0, 11))
        assertFalse(ParseUtils.isInDoubleQuotes(document, 0, 2))
        assertFalse(ParseUtils.isInDoubleQuotes(document, 0, 27))
    }
}