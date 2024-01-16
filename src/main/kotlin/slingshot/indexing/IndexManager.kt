/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.indexing

import kotlinx.serialization.json.Json
import net.jpountz.xxhash.XXHashFactory
import org.tinylog.kotlin.Logger
import slingshot.parsing.SvDocument
import java.nio.ByteBuffer
import java.nio.file.Path
import java.nio.file.Paths
import kotlin.io.path.absolute
import kotlin.io.path.absolutePathString
import kotlin.io.path.writeText

/**
 * This is the tool for managing the index cache of all files.
 *
 * Note: We use xxHash to keep track of file modifications, because the implementation is known,
 * fast and stable, whereas Java's String hashCode() is technically not stable across JVM versions.
 */
class IndexManager {
    /** Currently loaded index. Either empty if no index exists yet or partially filled. */
    private val index = Index(INDEX_VERSION)

    // currently we aren't supporting saving/writing yet like in the Rust version, because it slows down
    // programming and probably won't work that well anyway
    // just reindex the project on startup
    // TODO support full project indexing in future with cache saving to disk

    /**
     * Inserts a document with the specified absolute path [path] and contents [document].
     * The document hash is computed using xxHash64, if the document is already in the index,
     * it will not be inserted.
     */
    fun insert(path: Path, document: String) {
        // this part is mostly done for the sake of not flushing the index when serialisation was implemented,
        // and may be expensive -> TODO we should consider skipping
        val hash = HASHER.hash64().hash(ByteBuffer.wrap(document.toByteArray()), 0x1337L)
        val pathString = path.absolutePathString()
        val existing = index.hashes[pathString]
        if (existing != null && existing == hash) {
            Logger.trace("No need to insert document at $path with hash " +
             "${java.lang.Long.toHexString(hash)}: already exists")
            return
        }

        // update the index with the latest xxHash
        index.hashes[pathString] = hash
        // if the document is not yet in the index, insert it. then, update the contents and clear any
        // existing document trees which would now be invalid.
        index.documents.putIfAbsent(pathString, IndexEntry(document))?.apply {
            contents = document
            tree = null
        }
        Logger.trace("(Re)inserted document $path with hash ${java.lang.Long.toHexString(hash)}")
    }

    /**
     * Attempts to retrieve a document from the index.
     */
    fun retrieve(path: Path): IndexEntry? {
        val pathString = path.absolutePathString()
        return index.documents[pathString]
    }

    /**
     * Serialises the index to disk as JSON. The [baseDir] is the project base directory.
     * The serialised index is written to $XDG_DATA_HOME/slingshot/index_(basedir).json, where
     * (basedir) is the base directory with / replaced with $.
     */
    fun flush(baseDir: Path) {
        // FIXME this may not properly support paths with a dollar sign already in them
        val baseDirStr = baseDir.absolutePathString().replace("/", "$").replace("\\", "$")
        val home = System.getProperty("user.home")
        // this is $XDG_DATA_HOME
        val output = Paths.get(home, ".local", "share", "slingshot", "index_$baseDirStr.json")
        Logger.info("Flushing index to path: $output")

        val json = Json { allowStructuredMapKeys = true; prettyPrint = true }
        val document = json.encodeToString(Index.serializer(), index)
        output.writeText(document)
    }

    private fun ensureVersionCompatibility() {
        if (index.version != INDEX_VERSION) {
            throw IllegalArgumentException("On-disk index version ${index.version} is not compatible" +
             " with server index version $INDEX_VERSION")
        }
    }

    /**
     * Returns a list of all documents in the index.
     */
    fun getAllDocuments(): List<SvDocument> {
        return index.documents.map { it.value.tree }.filterNotNull()
    }

    companion object {
        private val HASHER = XXHashFactory.fastestInstance()
    }
}