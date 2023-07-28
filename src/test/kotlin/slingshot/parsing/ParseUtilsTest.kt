/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

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
}