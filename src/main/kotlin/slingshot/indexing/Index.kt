/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.indexing

import kotlinx.serialization.Serializable
import java.nio.file.Path
import java.util.concurrent.ConcurrentHashMap

/** Current Slingshot index version */
const val INDEX_VERSION = "1.0.0"

/**
 * This is the actual index that is written to disk. Note that the index, like clangd's, is per
 * project.
 * This is just serialised to disk with ObjectOutputStream so put whatever in here.
 */
@Serializable
data class Index(
    /** Version of the index file */
    val version: String,

    // note that we are using ConcurrentHashMap because Index is accessed from multiple threads

    /**
     * Mapping between the absolute path of each document in the project and the xxh3 hash of its contents.
     * Used to determine if the index needs to be refreshed when the LSP starts up or not.
     *
     * Note: This needs to be a String so that it's serialisable. The API still uses Path.
     */
    val hashes: MutableMap<String, Long> = ConcurrentHashMap(),

    /**
     * Stores a document path and the latest contents of that document we have on file. This is
     * used to assist in completion.
     *
     * Note: This needs to be a String so that it's serialisable. The API still uses Path.
     */
    val documents: MutableMap<String, IndexEntry> = ConcurrentHashMap()
) {

}