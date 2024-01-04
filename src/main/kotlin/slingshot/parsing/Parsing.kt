/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

package slingshot.parsing

import kotlinx.serialization.Serializable
import kotlinx.serialization.Transient

/** SystemVerilog token type */
@Serializable
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

/** Types of completions we can recommend to the user */
@Serializable
enum class CompletionTypes {
    None,

    /** A module in the current document */
    Module,

    /** A port from the current module the cursor is in */
    PortSameModule,

    /** A port from the module that is being instantiated */
    PortInstantiatedModule,

    /** A variable from the same module being instantiated */
    VariableSameModule,

    /** The name of a `typedef enum` */
    Enum,

    /** A constant in a `typedef enum` */
    EnumValue,

    /** A `defined macro */
    Macro,

    /** Either "posedge" or "negedge" */
    Edge,

    /** The keyword `logic` */
    Logic,

    /** Always blocks: `always_ff`, `always_comb`, `always_latch` */
    Always,

    /** An SV system task, e.g. $display, $error, etc */
    SystemTask,
}

/** A top level object in a SystemVerilog document */
interface SvTopLevelObject

class SvNull : SvTopLevelObject

/** A SvToken contains the name of the token and its type */
@Serializable
data class SvToken(
    val name: String,
    val tokenType: TokenType,
    @Transient val parent: SvTopLevelObject = SvNull()
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as SvToken

        return name == other.name
    }

    override fun hashCode(): Int {
        return name.hashCode()
    }
}

/**
 * A SystemVerilog module which contains a public set of ports and private set of variables
 * @param name name of the module
 * @param ports public module ports
 * @param variables private logic, wire, etc, tokens to this module
 * @param parent owner document
 */
@Serializable
data class SvModule(
    val name: String,
    val ports: MutableSet<SvToken>,
    val variables: MutableSet<SvToken>,
    @Transient val parent: SvDocument = SvDocument()
) : SvTopLevelObject {
    /**
     * Locates a port in this module by a partial string. This is used for auto-complete. Currently
     * this uses a naive slow algorithm but could be made more optimal in future.
     */
    fun findPort(port: String): SvToken? {
        return ports.firstOrNull { it.name.contains(port) }
    }

    fun findVar(variable: String): SvToken? {
        return variables.firstOrNull { it.name.contains(variable) }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as SvModule

        if (name != other.name) return false
        if (ports != other.ports) return false
        if (variables != other.variables) return false

        return true
    }

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + ports.hashCode()
        result = 31 * result + variables.hashCode()
        return result
    }
}

/** A SystemVerilog enum */
@Serializable
data class SvEnum(
    val name: String,
    val enumValues: MutableSet<SvToken>,
    @Transient val parent: SvDocument = SvDocument()
) : SvTopLevelObject {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as SvEnum

        if (name != other.name) return false
        if (enumValues != other.enumValues) return false

        return true
    }

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + enumValues.hashCode()
        return result
    }
}

/** A SystemVerilog `define macro, with an optional value */
@Serializable
data class SvMacro(val name: String, val value: String?) : SvTopLevelObject {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as SvMacro

        return name == other.name
    }

    override fun hashCode(): Int {
        return name.hashCode()
    }
}