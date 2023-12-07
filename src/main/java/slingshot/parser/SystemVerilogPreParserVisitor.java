// Generated from /home/matt/workspace/slingshot/src/main/antlr/SystemVerilogPreParser.g4 by ANTLR 4.12.0
package slingshot.parser;
import org.antlr.v4.runtime.tree.ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SystemVerilogPreParser}.
 *
 * @param <T> The return type of the visit operation. Use {@link Void} for
 * operations with no return type.
 */
public interface SystemVerilogPreParserVisitor<T> extends ParseTreeVisitor<T> {
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#source_text}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitSource_text(SystemVerilogPreParser.Source_textContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#compiler_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitCompiler_directive(SystemVerilogPreParser.Compiler_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#begin_keywords_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitBegin_keywords_directive(SystemVerilogPreParser.Begin_keywords_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#celldefine_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitCelldefine_directive(SystemVerilogPreParser.Celldefine_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#default_nettype_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitDefault_nettype_directive(SystemVerilogPreParser.Default_nettype_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#default_nettype_value}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitDefault_nettype_value(SystemVerilogPreParser.Default_nettype_valueContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#else_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitElse_directive(SystemVerilogPreParser.Else_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#elsif_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitElsif_directive(SystemVerilogPreParser.Elsif_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#end_keywords_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitEnd_keywords_directive(SystemVerilogPreParser.End_keywords_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#endcelldefine_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitEndcelldefine_directive(SystemVerilogPreParser.Endcelldefine_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#endif_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitEndif_directive(SystemVerilogPreParser.Endif_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#file_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitFile_directive(SystemVerilogPreParser.File_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#filename}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitFilename(SystemVerilogPreParser.FilenameContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#group_of_lines}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitGroup_of_lines(SystemVerilogPreParser.Group_of_linesContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#identifier}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitIdentifier(SystemVerilogPreParser.IdentifierContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#ifdef_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitIfdef_directive(SystemVerilogPreParser.Ifdef_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#ifndef_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitIfndef_directive(SystemVerilogPreParser.Ifndef_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#include_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitInclude_directive(SystemVerilogPreParser.Include_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#level}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLevel(SystemVerilogPreParser.LevelContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#line_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLine_directive(SystemVerilogPreParser.Line_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#line_directive_}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitLine_directive_(SystemVerilogPreParser.Line_directive_Context ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_delimiter}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_delimiter(SystemVerilogPreParser.Macro_delimiterContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_esc_newline}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_esc_newline(SystemVerilogPreParser.Macro_esc_newlineContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_esc_quote}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_esc_quote(SystemVerilogPreParser.Macro_esc_quoteContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_identifier}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_identifier(SystemVerilogPreParser.Macro_identifierContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_name}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_name(SystemVerilogPreParser.Macro_nameContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_quote}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_quote(SystemVerilogPreParser.Macro_quoteContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_text}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_text(SystemVerilogPreParser.Macro_textContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_text_}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_text_(SystemVerilogPreParser.Macro_text_Context ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#macro_usage}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitMacro_usage(SystemVerilogPreParser.Macro_usageContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#nounconnected_drive_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitNounconnected_drive_directive(SystemVerilogPreParser.Nounconnected_drive_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#number}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitNumber(SystemVerilogPreParser.NumberContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#pragma_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitPragma_directive(SystemVerilogPreParser.Pragma_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#pragma_expression}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitPragma_expression(SystemVerilogPreParser.Pragma_expressionContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#pragma_keyword}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitPragma_keyword(SystemVerilogPreParser.Pragma_keywordContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#pragma_name}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitPragma_name(SystemVerilogPreParser.Pragma_nameContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#pragma_value}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitPragma_value(SystemVerilogPreParser.Pragma_valueContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#resetall_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitResetall_directive(SystemVerilogPreParser.Resetall_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#source_text_}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitSource_text_(SystemVerilogPreParser.Source_text_Context ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#string_literal}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitString_literal(SystemVerilogPreParser.String_literalContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#text_macro_definition}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitText_macro_definition(SystemVerilogPreParser.Text_macro_definitionContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#text_macro_usage}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitText_macro_usage(SystemVerilogPreParser.Text_macro_usageContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#time_precision}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitTime_precision(SystemVerilogPreParser.Time_precisionContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#time_unit}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitTime_unit(SystemVerilogPreParser.Time_unitContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#timescale_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitTimescale_directive(SystemVerilogPreParser.Timescale_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#unconnected_drive_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitUnconnected_drive_directive(SystemVerilogPreParser.Unconnected_drive_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#unconnected_drive_value}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitUnconnected_drive_value(SystemVerilogPreParser.Unconnected_drive_valueContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#undef_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitUndef_directive(SystemVerilogPreParser.Undef_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#undefineall_directive}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitUndefineall_directive(SystemVerilogPreParser.Undefineall_directiveContext ctx);
	/**
	 * Visit a parse tree produced by {@link SystemVerilogPreParser#version_specifier}.
	 * @param ctx the parse tree
	 * @return the visitor result
	 */
	T visitVersion_specifier(SystemVerilogPreParser.Version_specifierContext ctx);
}