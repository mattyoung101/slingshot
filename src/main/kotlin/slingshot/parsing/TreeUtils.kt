/*
 * Copyright (c) 2018 GRosenberg.
 * Copyright (c) 2024 Matt Young.
 *
 * Source: https://stackoverflow.com/a/50068645/5007892
 */
package slingshot.parsing

import org.antlr.v4.runtime.RuleContext
import org.antlr.v4.runtime.misc.Utils
import org.antlr.v4.runtime.tree.TerminalNode
import org.antlr.v4.runtime.tree.Tree
import org.antlr.v4.runtime.tree.Trees

object TreeUtils {
    /** Platform dependent end-of-line marker  */
    private val Eol: String = System.lineSeparator()

    /** The literal indent char(s) used for pretty-printing  */
    private const val Indents: String = "  "
    private var level = 0

    /**
     * Pretty print out a whole tree. [.getNodeText] is used on the node payloads to get the text
     * for the nodes. (Derived from Trees.toStringTree(....))
     */
    fun toPrettyTree(t: Tree, ruleNames: List<String>): String {
        level = 0
        return process(t, ruleNames).replace("(?m)^\\s+$".toRegex(), "")
            .replace("\\r?\\n\\r?\\n".toRegex(), Eol)
    }

    private fun getNodeLocation(t: Tree): String {
        return when (t) {
            is RuleContext -> {
                "<${t.sourceInterval}>"
            }
            is TerminalNode -> {
                "<${t.sourceInterval}>"
            }
            else -> {
                "<range unknown for ${t.javaClass}>"
            }
        }
    }

    private fun process(t: Tree, ruleNames: List<String>): String {
        if (t.childCount == 0) return Utils.escapeWhitespace(Trees.getNodeText(t, ruleNames), false)
        val sb = StringBuilder()
        sb.append(lead(level))
        level++
        val s = Utils.escapeWhitespace(Trees.getNodeText(t, ruleNames), false)
        sb.append(s).append("    ").append(getNodeLocation(t))
        for (i in 0..<t.childCount) {
            sb.append(process(t.getChild(i), ruleNames))
        }
        level--
        sb.append(lead(level))
        return sb.toString()
    }

    private fun lead(level: Int): String {
        val sb = StringBuilder()
        if (level > 0) {
            sb.append(Eol)
            sb.append(Indents.repeat(level))
        }
        return sb.toString()
    }
}