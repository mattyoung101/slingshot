/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.config

import com.charleskorn.kaml.Yaml
import org.tinylog.kotlin.Logger
import java.net.URI
import java.nio.file.Path
import kotlin.io.path.exists
import kotlin.io.path.readText
import kotlin.io.path.toPath

object ConfigUtils {
    /** Tries to load and deserialise a Slingshot config from the project root directory. */
    fun loadConfigFromPath(path: Path): SlingshotConfig? {
        val configYamlPath = path.resolve(".slingshot.yaml")
        val configYmlPath = path.resolve(".slingshot.yml")

        try {
            if (configYamlPath.exists()) {
                // deserialise
                return Yaml.default.decodeFromString(SlingshotConfig.serializer(), configYamlPath.readText())
            } else if (configYmlPath.exists()) {
                // deserialise
                return Yaml.default.decodeFromString(SlingshotConfig.serializer(), configYmlPath.readText())
            } else {
                // can't find it
                Logger.error("Cannot find Slingshot config! Tried:\n- $configYamlPath" +
                 "\n- $configYmlPath\n...but neither exist!")
                return null
            }
        } catch (e: IllegalArgumentException) {
            Logger.error("Failed to parse config YAML:\n${e.stackTraceToString().trim()}")
            Logger.error("Note: Tried either $configYamlPath or $configYmlPath")
            return null
        }
    }
}