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


/**
 * A SystemVerilog document (contains a set of modules and enums). This is used to build a very
 * simplified internal representation of an SV document for completion and indexing purposes. For
 * more information, see docs/index_design.md which covers this as well.
 */
data class SvDocument(
    val modules: MutableSet<SvModule> = mutableSetOf(),
    val enums: MutableSet<SvEnum> = mutableSetOf(),
    val macros: MutableSet<SvMacro> = mutableSetOf()
) {
    /** Currently processing SV module */
    private var curModule: SvModule? = null
    /** Currently processing SV enum typedef */
    private var curEnum: SvEnum? = null

    /**
     * Starts a new module in the document if one is currently not started, otherwise, ends the
     * existing module.
     */
    fun newModule(name: String) {
        if (curEnum != null) return Logger.warn("Cannot start a new module when an enum is active!")
        finishModule()
        Logger.debug("Starting new module: $name")
        curModule = SvModule(name, mutableSetOf(), mutableSetOf(), this)
    }

    /** Finishes the current module if and only if one is active */
    fun finishModule() {
        // it's OK to attempt to finish a module that doesn't exist, we just won't do anything
        curModule ?: return
        Logger.debug("Finishing current module: ${curModule?.name}")
        modules.add(curModule!!)
        curModule = null
    }

    fun addVariable(name: String) {
        curModule ?: return Logger.debug("Trying to add variable $name, but no module is active!")
        Logger.debug("    Adding variable: $name to module: ${curModule!!.name}")
        curModule!!.variables.add(SvToken(name, TokenType.Variable, curModule!!))
    }

    fun addPort(name: String) {
        curModule ?: return Logger.debug("Trying to add port $name, but no module is active!")
        Logger.debug("    Adding port: $name to module: ${curModule!!.name}")
        curModule!!.ports.add(SvToken(name, TokenType.Port, curModule!!))
    }

    /**
     * Starts a new enum in the document if one is not currently started, otherwise, ends the existing enum.
     */
    fun newEnum(name: String) {
        if (curModule != null) return Logger.warn("Cannot start new enum when a module is active!")
        finishEnum()
        Logger.debug("Starting new enum: $name")
        curEnum = SvEnum(name, mutableSetOf(), this)
    }

    fun addEnumValue(name: String) {
        curEnum ?: Logger.debug("Trying to add enum value $name, but no enum is active!") // TODO maybe we just start
        Logger.debug("    Adding enum value: $name to enum: ${curEnum!!.name}")
        curEnum!!.enumValues.add(SvToken(name, TokenType.EnumValue, curEnum!!))
    }

    fun finishEnum() {
        curEnum ?: return
        Logger.debug("Finishing current enum: ${curEnum?.name}")
        enums.add(curEnum!!)
        curEnum = null
    }

    fun addMacro(name: String, value: String?) {
        Logger.debug("Adding macro $name with value: $value")
        macros.add(SvMacro(name, value))
    }

    /**
     * Returns true if the given variable token has already been declared in the current module as
     * a port.
     */
    fun isVarDeclaredAsPort(name: String): Boolean {
        return curModule?.ports?.any { it.name == name && it.tokenType == TokenType.Port } ?: false
    }

    fun getModuleByName(name: String): SvModule {
        return modules.first { it.name == name }
    }

    fun isEnumActive(): Boolean {
        return curEnum != null
    }
}
