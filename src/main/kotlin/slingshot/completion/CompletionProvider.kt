/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.completion

import slingshot.parsing.SvDocument
import slingshot.parsing.TokenType

/**
 * Interface to a piece of software that can generate completions, e.g. sv-parser, Slang, etc.
 */
interface CompletionProvider {
    /**
     * Parses a document to an [SvDocument] instance, and determines the token that the user currently has
     * active. Throws a [CompletionException] in case the document could not be parsed.
     * @param document current document
     * @param line current line (0 indexed)
     * @param pos current position in current line (0 indexed)
     */
    @Throws(CompletionException::class)
    fun parseDocument(document: String, line: Int, pos: Int): Pair<SvDocument, TokenType>
}
