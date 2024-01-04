/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.config

import kotlinx.serialization.Serializable

/** Current Slingshot config version */
const val CONFIG_VERSION = "1.0.0"

@Serializable
data class SlingshotConfig(
    val version: String,
    val includeDirs: List<String> = listOf(),
)
