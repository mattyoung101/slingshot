/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.completion

/**
 * Exception thrown in case completion failed
 */
class CompletionException : RuntimeException {
    constructor(): super()
    constructor(msg: String) : super(msg)
    constructor(ex: Exception): super(ex)
}