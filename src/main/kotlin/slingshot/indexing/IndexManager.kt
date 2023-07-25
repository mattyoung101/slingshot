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
    private val index = Index()

    // currently we aren't supporting saving/writing yet like in the Rust version, because it slows down
    // programming and probably won't work that well anyway
    // just reindex the project on startup

    fun insert(path: Path, document: String) {
        // this part is mostly done for the sake of not flushing the index when serialisation was implemented,
        // and may be expensive -> TODO we should consider skipping
        val hash = HASHER.hash64().hash(ByteBuffer.wrap(document.toByteArray()), 0x1337L)
        val existing = index.hashes[path]
        if (existing != null && existing == hash) {
            Logger.debug("No need to insert document at $path with hash " +
             "${java.lang.Long.toHexString(hash)}: already exists")
            return
        }

        // update the index with the latest xxHash
        index.hashes[path] = hash
        // if the document is not yet in the index, insert it. then, update the contents and clear any
        // existing document trees which would now be invalid.
        index.documents.putIfAbsent(path, IndexEntry(document))?.apply {
            contents = document
            tree = null
        }
        Logger.debug("(Re)inserted document $path with hash ${java.lang.Long.toHexString(hash)}")
    }

    fun attachTree(path: Path, tree: SvDocument) {
        index.documents[path] ?: return Logger.error("Attempted to attach document tree to non-existent " +
         "path $path!")
        index.documents[path]?.tree = tree
        Logger.debug("Attached document tree to path $path")
    }

    fun retrieve(path: Path): IndexEntry? {
        return index.documents[path]
    }

    companion object {
        private val HASHER = XXHashFactory.fastestInstance()
    }
}