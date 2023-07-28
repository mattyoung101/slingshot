/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import org.tinylog.kotlin.Logger
import slingshot.completion.CompletionException

/** Abstract SystemVerilog token type */
enum class TokenType {
    /** A token type that does not matter to us for completion */
    NotInterested,

    /** For completion: The cursor is in a token type that we don't understand */
    Unknown,

    /** A module */
    Module,

    /** A class */
    Class,

    /** An enum */
    Enum,

    /** A "variable": logic, wire or register */
    Variable,

    /** A port in a module */
    Port,

    /** A`defined macro */
    Macro,

    /** A value of an enum */
    EnumValue,
}

/** A generic SystemVerilog object */
interface SvObject

/** A SvToken contains the name of the token and its type */
data class SvToken(val name: String, val tokenType: TokenType, val parent: SvModule)

/**
 * A SystemVerilog module which contains a public set of ports and private set of variables
 * @param name name of the module
 * @param ports public module ports
 * @param variables private logic, wire, etc, tokens to this module
 * @param parent owner document
 */
data class SvModule(val name: String, val ports: MutableList<SvToken>, val variables: MutableList<SvToken>, val parent: SvDocument) {
    /**
     * Locates a port in this module by a partial string. This is used for auto-complete. Currently
     * this uses a naive slow algorithm but could be made more optimal in future.
     */
    fun findPort(port: String): SvToken? {
        return ports.firstOrNull { it.name.contains(port) }
    }

    fun findVar(variable: String): SvToken ? {
        return variables.firstOrNull { it.name.contains(variable) }
    }
}

/** A SystemVerilog enum */
data class SvEnum(val name: String, val enumValues: List<SvToken>, val parent: SvDocument)
