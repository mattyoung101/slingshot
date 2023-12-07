// Generated from /home/matt/workspace/slingshot/src/main/antlr/SystemVerilogPreParser.g4 by ANTLR 4.12.0
package slingshot.parser;
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link SystemVerilogPreParser}.
 */
public interface SystemVerilogPreParserListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#source_text}.
	 * @param ctx the parse tree
	 */
	void enterSource_text(SystemVerilogPreParser.Source_textContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#source_text}.
	 * @param ctx the parse tree
	 */
	void exitSource_text(SystemVerilogPreParser.Source_textContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#compiler_directive}.
	 * @param ctx the parse tree
	 */
	void enterCompiler_directive(SystemVerilogPreParser.Compiler_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#compiler_directive}.
	 * @param ctx the parse tree
	 */
	void exitCompiler_directive(SystemVerilogPreParser.Compiler_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#begin_keywords_directive}.
	 * @param ctx the parse tree
	 */
	void enterBegin_keywords_directive(SystemVerilogPreParser.Begin_keywords_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#begin_keywords_directive}.
	 * @param ctx the parse tree
	 */
	void exitBegin_keywords_directive(SystemVerilogPreParser.Begin_keywords_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#celldefine_directive}.
	 * @param ctx the parse tree
	 */
	void enterCelldefine_directive(SystemVerilogPreParser.Celldefine_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#celldefine_directive}.
	 * @param ctx the parse tree
	 */
	void exitCelldefine_directive(SystemVerilogPreParser.Celldefine_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#default_nettype_directive}.
	 * @param ctx the parse tree
	 */
	void enterDefault_nettype_directive(SystemVerilogPreParser.Default_nettype_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#default_nettype_directive}.
	 * @param ctx the parse tree
	 */
	void exitDefault_nettype_directive(SystemVerilogPreParser.Default_nettype_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#default_nettype_value}.
	 * @param ctx the parse tree
	 */
	void enterDefault_nettype_value(SystemVerilogPreParser.Default_nettype_valueContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#default_nettype_value}.
	 * @param ctx the parse tree
	 */
	void exitDefault_nettype_value(SystemVerilogPreParser.Default_nettype_valueContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#else_directive}.
	 * @param ctx the parse tree
	 */
	void enterElse_directive(SystemVerilogPreParser.Else_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#else_directive}.
	 * @param ctx the parse tree
	 */
	void exitElse_directive(SystemVerilogPreParser.Else_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#elsif_directive}.
	 * @param ctx the parse tree
	 */
	void enterElsif_directive(SystemVerilogPreParser.Elsif_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#elsif_directive}.
	 * @param ctx the parse tree
	 */
	void exitElsif_directive(SystemVerilogPreParser.Elsif_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#end_keywords_directive}.
	 * @param ctx the parse tree
	 */
	void enterEnd_keywords_directive(SystemVerilogPreParser.End_keywords_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#end_keywords_directive}.
	 * @param ctx the parse tree
	 */
	void exitEnd_keywords_directive(SystemVerilogPreParser.End_keywords_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#endcelldefine_directive}.
	 * @param ctx the parse tree
	 */
	void enterEndcelldefine_directive(SystemVerilogPreParser.Endcelldefine_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#endcelldefine_directive}.
	 * @param ctx the parse tree
	 */
	void exitEndcelldefine_directive(SystemVerilogPreParser.Endcelldefine_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#endif_directive}.
	 * @param ctx the parse tree
	 */
	void enterEndif_directive(SystemVerilogPreParser.Endif_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#endif_directive}.
	 * @param ctx the parse tree
	 */
	void exitEndif_directive(SystemVerilogPreParser.Endif_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#file_directive}.
	 * @param ctx the parse tree
	 */
	void enterFile_directive(SystemVerilogPreParser.File_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#file_directive}.
	 * @param ctx the parse tree
	 */
	void exitFile_directive(SystemVerilogPreParser.File_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#filename}.
	 * @param ctx the parse tree
	 */
	void enterFilename(SystemVerilogPreParser.FilenameContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#filename}.
	 * @param ctx the parse tree
	 */
	void exitFilename(SystemVerilogPreParser.FilenameContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#group_of_lines}.
	 * @param ctx the parse tree
	 */
	void enterGroup_of_lines(SystemVerilogPreParser.Group_of_linesContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#group_of_lines}.
	 * @param ctx the parse tree
	 */
	void exitGroup_of_lines(SystemVerilogPreParser.Group_of_linesContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#identifier}.
	 * @param ctx the parse tree
	 */
	void enterIdentifier(SystemVerilogPreParser.IdentifierContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#identifier}.
	 * @param ctx the parse tree
	 */
	void exitIdentifier(SystemVerilogPreParser.IdentifierContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#ifdef_directive}.
	 * @param ctx the parse tree
	 */
	void enterIfdef_directive(SystemVerilogPreParser.Ifdef_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#ifdef_directive}.
	 * @param ctx the parse tree
	 */
	void exitIfdef_directive(SystemVerilogPreParser.Ifdef_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#ifndef_directive}.
	 * @param ctx the parse tree
	 */
	void enterIfndef_directive(SystemVerilogPreParser.Ifndef_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#ifndef_directive}.
	 * @param ctx the parse tree
	 */
	void exitIfndef_directive(SystemVerilogPreParser.Ifndef_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#include_directive}.
	 * @param ctx the parse tree
	 */
	void enterInclude_directive(SystemVerilogPreParser.Include_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#include_directive}.
	 * @param ctx the parse tree
	 */
	void exitInclude_directive(SystemVerilogPreParser.Include_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#level}.
	 * @param ctx the parse tree
	 */
	void enterLevel(SystemVerilogPreParser.LevelContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#level}.
	 * @param ctx the parse tree
	 */
	void exitLevel(SystemVerilogPreParser.LevelContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#line_directive}.
	 * @param ctx the parse tree
	 */
	void enterLine_directive(SystemVerilogPreParser.Line_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#line_directive}.
	 * @param ctx the parse tree
	 */
	void exitLine_directive(SystemVerilogPreParser.Line_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#line_directive_}.
	 * @param ctx the parse tree
	 */
	void enterLine_directive_(SystemVerilogPreParser.Line_directive_Context ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#line_directive_}.
	 * @param ctx the parse tree
	 */
	void exitLine_directive_(SystemVerilogPreParser.Line_directive_Context ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_delimiter}.
	 * @param ctx the parse tree
	 */
	void enterMacro_delimiter(SystemVerilogPreParser.Macro_delimiterContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_delimiter}.
	 * @param ctx the parse tree
	 */
	void exitMacro_delimiter(SystemVerilogPreParser.Macro_delimiterContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_esc_newline}.
	 * @param ctx the parse tree
	 */
	void enterMacro_esc_newline(SystemVerilogPreParser.Macro_esc_newlineContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_esc_newline}.
	 * @param ctx the parse tree
	 */
	void exitMacro_esc_newline(SystemVerilogPreParser.Macro_esc_newlineContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_esc_quote}.
	 * @param ctx the parse tree
	 */
	void enterMacro_esc_quote(SystemVerilogPreParser.Macro_esc_quoteContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_esc_quote}.
	 * @param ctx the parse tree
	 */
	void exitMacro_esc_quote(SystemVerilogPreParser.Macro_esc_quoteContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_identifier}.
	 * @param ctx the parse tree
	 */
	void enterMacro_identifier(SystemVerilogPreParser.Macro_identifierContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_identifier}.
	 * @param ctx the parse tree
	 */
	void exitMacro_identifier(SystemVerilogPreParser.Macro_identifierContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_name}.
	 * @param ctx the parse tree
	 */
	void enterMacro_name(SystemVerilogPreParser.Macro_nameContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_name}.
	 * @param ctx the parse tree
	 */
	void exitMacro_name(SystemVerilogPreParser.Macro_nameContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_quote}.
	 * @param ctx the parse tree
	 */
	void enterMacro_quote(SystemVerilogPreParser.Macro_quoteContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_quote}.
	 * @param ctx the parse tree
	 */
	void exitMacro_quote(SystemVerilogPreParser.Macro_quoteContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_text}.
	 * @param ctx the parse tree
	 */
	void enterMacro_text(SystemVerilogPreParser.Macro_textContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_text}.
	 * @param ctx the parse tree
	 */
	void exitMacro_text(SystemVerilogPreParser.Macro_textContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_text_}.
	 * @param ctx the parse tree
	 */
	void enterMacro_text_(SystemVerilogPreParser.Macro_text_Context ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_text_}.
	 * @param ctx the parse tree
	 */
	void exitMacro_text_(SystemVerilogPreParser.Macro_text_Context ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#macro_usage}.
	 * @param ctx the parse tree
	 */
	void enterMacro_usage(SystemVerilogPreParser.Macro_usageContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#macro_usage}.
	 * @param ctx the parse tree
	 */
	void exitMacro_usage(SystemVerilogPreParser.Macro_usageContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#nounconnected_drive_directive}.
	 * @param ctx the parse tree
	 */
	void enterNounconnected_drive_directive(SystemVerilogPreParser.Nounconnected_drive_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#nounconnected_drive_directive}.
	 * @param ctx the parse tree
	 */
	void exitNounconnected_drive_directive(SystemVerilogPreParser.Nounconnected_drive_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#number}.
	 * @param ctx the parse tree
	 */
	void enterNumber(SystemVerilogPreParser.NumberContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#number}.
	 * @param ctx the parse tree
	 */
	void exitNumber(SystemVerilogPreParser.NumberContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#pragma_directive}.
	 * @param ctx the parse tree
	 */
	void enterPragma_directive(SystemVerilogPreParser.Pragma_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#pragma_directive}.
	 * @param ctx the parse tree
	 */
	void exitPragma_directive(SystemVerilogPreParser.Pragma_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#pragma_expression}.
	 * @param ctx the parse tree
	 */
	void enterPragma_expression(SystemVerilogPreParser.Pragma_expressionContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#pragma_expression}.
	 * @param ctx the parse tree
	 */
	void exitPragma_expression(SystemVerilogPreParser.Pragma_expressionContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#pragma_keyword}.
	 * @param ctx the parse tree
	 */
	void enterPragma_keyword(SystemVerilogPreParser.Pragma_keywordContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#pragma_keyword}.
	 * @param ctx the parse tree
	 */
	void exitPragma_keyword(SystemVerilogPreParser.Pragma_keywordContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#pragma_name}.
	 * @param ctx the parse tree
	 */
	void enterPragma_name(SystemVerilogPreParser.Pragma_nameContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#pragma_name}.
	 * @param ctx the parse tree
	 */
	void exitPragma_name(SystemVerilogPreParser.Pragma_nameContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#pragma_value}.
	 * @param ctx the parse tree
	 */
	void enterPragma_value(SystemVerilogPreParser.Pragma_valueContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#pragma_value}.
	 * @param ctx the parse tree
	 */
	void exitPragma_value(SystemVerilogPreParser.Pragma_valueContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#resetall_directive}.
	 * @param ctx the parse tree
	 */
	void enterResetall_directive(SystemVerilogPreParser.Resetall_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#resetall_directive}.
	 * @param ctx the parse tree
	 */
	void exitResetall_directive(SystemVerilogPreParser.Resetall_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#source_text_}.
	 * @param ctx the parse tree
	 */
	void enterSource_text_(SystemVerilogPreParser.Source_text_Context ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#source_text_}.
	 * @param ctx the parse tree
	 */
	void exitSource_text_(SystemVerilogPreParser.Source_text_Context ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#string_literal}.
	 * @param ctx the parse tree
	 */
	void enterString_literal(SystemVerilogPreParser.String_literalContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#string_literal}.
	 * @param ctx the parse tree
	 */
	void exitString_literal(SystemVerilogPreParser.String_literalContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#text_macro_definition}.
	 * @param ctx the parse tree
	 */
	void enterText_macro_definition(SystemVerilogPreParser.Text_macro_definitionContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#text_macro_definition}.
	 * @param ctx the parse tree
	 */
	void exitText_macro_definition(SystemVerilogPreParser.Text_macro_definitionContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#text_macro_usage}.
	 * @param ctx the parse tree
	 */
	void enterText_macro_usage(SystemVerilogPreParser.Text_macro_usageContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#text_macro_usage}.
	 * @param ctx the parse tree
	 */
	void exitText_macro_usage(SystemVerilogPreParser.Text_macro_usageContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#time_precision}.
	 * @param ctx the parse tree
	 */
	void enterTime_precision(SystemVerilogPreParser.Time_precisionContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#time_precision}.
	 * @param ctx the parse tree
	 */
	void exitTime_precision(SystemVerilogPreParser.Time_precisionContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#time_unit}.
	 * @param ctx the parse tree
	 */
	void enterTime_unit(SystemVerilogPreParser.Time_unitContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#time_unit}.
	 * @param ctx the parse tree
	 */
	void exitTime_unit(SystemVerilogPreParser.Time_unitContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#timescale_directive}.
	 * @param ctx the parse tree
	 */
	void enterTimescale_directive(SystemVerilogPreParser.Timescale_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#timescale_directive}.
	 * @param ctx the parse tree
	 */
	void exitTimescale_directive(SystemVerilogPreParser.Timescale_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#unconnected_drive_directive}.
	 * @param ctx the parse tree
	 */
	void enterUnconnected_drive_directive(SystemVerilogPreParser.Unconnected_drive_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#unconnected_drive_directive}.
	 * @param ctx the parse tree
	 */
	void exitUnconnected_drive_directive(SystemVerilogPreParser.Unconnected_drive_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#unconnected_drive_value}.
	 * @param ctx the parse tree
	 */
	void enterUnconnected_drive_value(SystemVerilogPreParser.Unconnected_drive_valueContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#unconnected_drive_value}.
	 * @param ctx the parse tree
	 */
	void exitUnconnected_drive_value(SystemVerilogPreParser.Unconnected_drive_valueContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#undef_directive}.
	 * @param ctx the parse tree
	 */
	void enterUndef_directive(SystemVerilogPreParser.Undef_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#undef_directive}.
	 * @param ctx the parse tree
	 */
	void exitUndef_directive(SystemVerilogPreParser.Undef_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#undefineall_directive}.
	 * @param ctx the parse tree
	 */
	void enterUndefineall_directive(SystemVerilogPreParser.Undefineall_directiveContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#undefineall_directive}.
	 * @param ctx the parse tree
	 */
	void exitUndefineall_directive(SystemVerilogPreParser.Undefineall_directiveContext ctx);
	/**
	 * Enter a parse tree produced by {@link SystemVerilogPreParser#version_specifier}.
	 * @param ctx the parse tree
	 */
	void enterVersion_specifier(SystemVerilogPreParser.Version_specifierContext ctx);
	/**
	 * Exit a parse tree produced by {@link SystemVerilogPreParser#version_specifier}.
	 * @param ctx the parse tree
	 */
	void exitVersion_specifier(SystemVerilogPreParser.Version_specifierContext ctx);
}