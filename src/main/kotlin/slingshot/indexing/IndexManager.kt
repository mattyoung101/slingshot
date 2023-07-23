/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.indexing

import net.jpountz.xxhash.XXHashFactory
import org.tinylog.kotlin.Logger
import slingshot.parsing.SvDocument
import java.nio.ByteBuffer
import java.nio.file.Path

/**
 * This is the tool for managing the index cache of all files.
 */
class IndexManager {
    /** Currently loaded index. Either empty if no index exists yet or partially filled. */
    val index = Index()

    // currently we aren't supporting saving/writing yet like in the Rust version, because it slows down
    // programming and probably won't work that well anyway
    // just reindex the project on startup

    fun insert(path: Path, document: String, documentTree: SvDocument) {
        val hash = XXHashFactory.fastestInstance().hash64().hash(ByteBuffer.wrap(document.toByteArray()),
        0x1337L)
        val existing = index.documentHashes[path]

        if (existing != null && existing == hash) {
            Logger.debug("No need to insert document at $path with hash $hash: already exists")
            return
        }

        index.documentHashes[path] = hash
        index.documentTrees[path] = documentTree
        index.documentContents[path] = document
        Logger.debug("Inserted document at path $path with hash $hash into index")
    }
}