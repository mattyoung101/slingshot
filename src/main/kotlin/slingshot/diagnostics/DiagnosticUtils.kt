/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.diagnostics

import java.nio.file.Path
import java.nio.file.Paths
import kotlin.io.path.extension
import kotlin.io.path.forEachDirectoryEntry

object DiagnosticUtils {
    private val SV_EXTENSIONS = listOf("sv", "v", "svh")

    /**
     * Walks a list of paths, looking for *.v, *.sv, *.svh files, and returns the list of these files.
     * This is not recursive, so it doesn't visit sub-dirs.
     * Used for whole-project indexing.
     */
    fun walkIncludeDirs(dirs: List<Path>): List<Path> {
        val out = mutableListOf<Path>()

        for (dir in dirs) {
            dir.forEachDirectoryEntry {
                if (it.extension in SV_EXTENSIONS) {
                    out.add(it)
                }
            }
        }

        return out
    }
}