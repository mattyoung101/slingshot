/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.diagnostics

import org.tinylog.kotlin.Logger
import java.nio.file.FileSystems
import java.nio.file.Path
import kotlin.io.path.forEachDirectoryEntry
import kotlin.io.path.isDirectory

object DiagnosticUtils {
    private val BLACKLIST = listOf(".git")

    private fun isInBlacklist(path: Path): Boolean {
        for (entry in BLACKLIST) {
            if (path.toString().contains(entry)) {
                return true
            }
        }
        return false
    }

    fun findFilesByGlobs(dir: Path, globs: List<String>): List<Path> {
        val out = mutableListOf<Path>()

        for (configGlob in globs) {
            // implicit `**/` is required to correctly match subdirectories
            val shouldAddImplicitAsterisk = !configGlob.startsWith("**") && configGlob.contains("/")
            if (shouldAddImplicitAsterisk) {
                Logger.debug("Adding implicit **/ to glob $configGlob")
            }

            // construct real glob using base dir
            val glob = "glob:${if (shouldAddImplicitAsterisk) "**/" else ""}$configGlob"
            Logger.debug("Trying glob: $glob")
            val matcher = FileSystems.getDefault().getPathMatcher(glob)

            // iterate over directory, recursively calling out to sub-directories
            // in effect, a BFS
            dir.forEachDirectoryEntry {
                if (isInBlacklist(it)) return@forEachDirectoryEntry

                if (it.isDirectory()) {
                    // recurse
                    Logger.debug("Recurse to sub-directory: $it")
                    out.addAll(findFilesByGlobs(it, globs))
                } else {
                    // base case
                    if (matcher.matches(it)) {
                        Logger.trace("    Found match! $it")
                        out.add(it)
                    } else {
                        Logger.trace("    File $it does not match glob $glob")
                    }
                }
            }
        }
        return out
    }
}

//fun main(args: Array<String>) {
//    val config = SlingshotConfig(version="1.0.0", globs=listOf("rtl/*.sv", "rtl/*.svh"))
//    DiagnosticUtils.findFilesByGlobs(Paths.get("/home/matt/workspace/aspire_rv32"), config.globs)
//}