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

/** A SvToken contains the name of the token and its type */
data class SvToken(val name: String, val tokenType: TokenType)

/**
 * A SystemVerilog module which contains a public set of ports and private set of variables
 * @param name name of the module
 * @param ports public module ports
 * @param variables private logic, wire, etc, tokens to this module
 */
data class SvModule(val name: String, val ports: MutableList<SvToken>, val variables: MutableList<SvToken>)

/** A SystemVerilog enum */
data class SvEnum(val name: String, val enumValues: List<SvToken>)

/**
 * A SystemVerilog document (contains a set of modules and enums). This is used to build a very
 * simplified internal representation of an SV document for completion and indexing purposes. For
 * more information, see docs/index_design.md which covers this as well.
 */
data class SvDocument(
    val modules: MutableList<SvModule> = mutableListOf(),
    val enums: MutableList<SvEnum> = mutableListOf()
) {
    private var curModule: SvModule? = null

    /**
     * Starts a new module in the document if one is currently not started, otherwise, ends the
     * existing module.
     */
    fun newModule(name: String) {
        finishModule()
        Logger.debug("Starting new module: $name")
        curModule = SvModule(name, mutableListOf(), mutableListOf())
    }

    /** Finishes the current module if and only if one is active */
    fun finishModule() {
        // it's OK to attempt to finish a module that doesn't exist, we just won't do anything
        curModule ?: return

        Logger.debug("Finishing current module: ${curModule?.name}")
        modules.add(curModule!!)
        curModule = null
    }

    fun addVariable(variable: SvToken) {
        curModule ?: throw CompletionException("Trying to add variable $variable, but no module is active!")

        Logger.debug("    Adding variable: ${variable.name} to module: ${curModule!!.name}")
        curModule!!.variables.add(variable)
    }

    fun addPort(port: SvToken) {
        curModule ?: throw CompletionException("Trying to add port $port, but no module is active!")

        Logger.debug("    Adding port: ${port.name} to module: ${curModule!!.name}")
        curModule!!.ports.add(port)
    }

    /**
     * Returns true if the given variable token has already been declared in the current module as
     * a port.
     */
    fun isVarDeclaredAsPort(variable: SvToken): Boolean {
        return curModule?.ports?.any { it.name == variable.name && it.tokenType == TokenType.Variable }
            ?: false
    }

    /**
     * Locates a port in a document by a partial string. This is used for auto-complete. Currently
     * this uses a naive slow algorithm but could be made more optimal in future.
     * Future optimisation plans would be to use trie_rs to compute a trie that maps between a
     * name and a module.
     * We could also do fuzzy-finding: https://github.com/lotabout/fuzzy-matcher
     */
    fun locatePort(port: String): Pair<SvModule, SvToken>? {
        for (module in modules) {
            for (p in module.ports) {
                if (p.name.contains(port)) {
                    return Pair(module, p)
                }
            }
        }
        return null
    }
}