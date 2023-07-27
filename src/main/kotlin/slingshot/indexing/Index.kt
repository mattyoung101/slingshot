/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.indexing

import slingshot.parsing.SvDocument
import java.io.Serializable
import java.nio.file.Path
import java.util.concurrent.ConcurrentHashMap

/** Current Slingshot index version */
const val INDEX_VERSION = "0.1.0"

/**
 * This is the actual index that is written to disk. Note that the index, like clangd's, is per
 * project.
 * This is just serialised to disk with ObjectOutputStream so put whatever in here.
 */
data class Index(
    /** Version of the index file */
    val version: String = INDEX_VERSION,

    // note that we are using ConcurrentHashMap because Index is accessed from multiple threads

    /**
     * Mapping between the absolute path of each document in the project and the xxh3 hash of its contents.
     * Used to determine if the index needs to be refreshed when the LSP starts up or not.
     */
    val hashes: MutableMap<Path, Long> = ConcurrentHashMap(),

    /**
     * Stores a document path and the latest contents of that document we have on file. This is
     * used to assist in completion.
     */
    val documents: MutableMap<Path, IndexEntry> = ConcurrentHashMap()
) {

}