/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.indexing

import slingshot.parser.SystemVerilogParser
import slingshot.parsing.SvDocument

/**
 * The [IndexEntry] stores a partial or complete entry for a SystemVerilog document in the index.
 * @param contents latest text contents of the document, must be filled
 * @param tree latest parsed tree of the document, doesn't have to be present
 */
data class IndexEntry(
    @Transient var contents: String,
    var tree: SvDocument? = null,
) {
}