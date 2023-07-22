#![allow(nonstandard_style)]
// Generated from SystemVerilogParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::systemverilogparser::*;

pub trait SystemVerilogParserListener<'input> : ParseTreeListener<'input,SystemVerilogParserContextType>{
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#library_text}.
 * @param ctx the parse tree
 */
fn enter_library_text(&mut self, _ctx: &Library_textContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#library_text}.
 * @param ctx the parse tree
 */
fn exit_library_text(&mut self, _ctx: &Library_textContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#library_description}.
 * @param ctx the parse tree
 */
fn enter_library_description(&mut self, _ctx: &Library_descriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#library_description}.
 * @param ctx the parse tree
 */
fn exit_library_description(&mut self, _ctx: &Library_descriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#library_declaration}.
 * @param ctx the parse tree
 */
fn enter_library_declaration(&mut self, _ctx: &Library_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#library_declaration}.
 * @param ctx the parse tree
 */
fn exit_library_declaration(&mut self, _ctx: &Library_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#library_incdir}.
 * @param ctx the parse tree
 */
fn enter_library_incdir(&mut self, _ctx: &Library_incdirContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#library_incdir}.
 * @param ctx the parse tree
 */
fn exit_library_incdir(&mut self, _ctx: &Library_incdirContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#include_statement}.
 * @param ctx the parse tree
 */
fn enter_include_statement(&mut self, _ctx: &Include_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#include_statement}.
 * @param ctx the parse tree
 */
fn exit_include_statement(&mut self, _ctx: &Include_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#file_path_spec}.
 * @param ctx the parse tree
 */
fn enter_file_path_spec(&mut self, _ctx: &File_path_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#file_path_spec}.
 * @param ctx the parse tree
 */
fn exit_file_path_spec(&mut self, _ctx: &File_path_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#source_text}.
 * @param ctx the parse tree
 */
fn enter_source_text(&mut self, _ctx: &Source_textContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#source_text}.
 * @param ctx the parse tree
 */
fn exit_source_text(&mut self, _ctx: &Source_textContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#description}.
 * @param ctx the parse tree
 */
fn enter_description(&mut self, _ctx: &DescriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#description}.
 * @param ctx the parse tree
 */
fn exit_description(&mut self, _ctx: &DescriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_header}.
 * @param ctx the parse tree
 */
fn enter_module_header(&mut self, _ctx: &Module_headerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_header}.
 * @param ctx the parse tree
 */
fn exit_module_header(&mut self, _ctx: &Module_headerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_declaration}.
 * @param ctx the parse tree
 */
fn enter_module_declaration(&mut self, _ctx: &Module_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_declaration}.
 * @param ctx the parse tree
 */
fn exit_module_declaration(&mut self, _ctx: &Module_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_name}.
 * @param ctx the parse tree
 */
fn enter_module_name(&mut self, _ctx: &Module_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_name}.
 * @param ctx the parse tree
 */
fn exit_module_name(&mut self, _ctx: &Module_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_keyword}.
 * @param ctx the parse tree
 */
fn enter_module_keyword(&mut self, _ctx: &Module_keywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_keyword}.
 * @param ctx the parse tree
 */
fn exit_module_keyword(&mut self, _ctx: &Module_keywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_declaration}.
 * @param ctx the parse tree
 */
fn enter_interface_declaration(&mut self, _ctx: &Interface_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_declaration}.
 * @param ctx the parse tree
 */
fn exit_interface_declaration(&mut self, _ctx: &Interface_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_name}.
 * @param ctx the parse tree
 */
fn enter_interface_name(&mut self, _ctx: &Interface_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_name}.
 * @param ctx the parse tree
 */
fn exit_interface_name(&mut self, _ctx: &Interface_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_header}.
 * @param ctx the parse tree
 */
fn enter_interface_header(&mut self, _ctx: &Interface_headerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_header}.
 * @param ctx the parse tree
 */
fn exit_interface_header(&mut self, _ctx: &Interface_headerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#program_declaration}.
 * @param ctx the parse tree
 */
fn enter_program_declaration(&mut self, _ctx: &Program_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#program_declaration}.
 * @param ctx the parse tree
 */
fn exit_program_declaration(&mut self, _ctx: &Program_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#program_name}.
 * @param ctx the parse tree
 */
fn enter_program_name(&mut self, _ctx: &Program_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#program_name}.
 * @param ctx the parse tree
 */
fn exit_program_name(&mut self, _ctx: &Program_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#program_header}.
 * @param ctx the parse tree
 */
fn enter_program_header(&mut self, _ctx: &Program_headerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#program_header}.
 * @param ctx the parse tree
 */
fn exit_program_header(&mut self, _ctx: &Program_headerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_declaration}.
 * @param ctx the parse tree
 */
fn enter_checker_declaration(&mut self, _ctx: &Checker_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_declaration}.
 * @param ctx the parse tree
 */
fn exit_checker_declaration(&mut self, _ctx: &Checker_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_name}.
 * @param ctx the parse tree
 */
fn enter_checker_name(&mut self, _ctx: &Checker_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_name}.
 * @param ctx the parse tree
 */
fn exit_checker_name(&mut self, _ctx: &Checker_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_ports}.
 * @param ctx the parse tree
 */
fn enter_checker_ports(&mut self, _ctx: &Checker_portsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_ports}.
 * @param ctx the parse tree
 */
fn exit_checker_ports(&mut self, _ctx: &Checker_portsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_decl_item}.
 * @param ctx the parse tree
 */
fn enter_checker_decl_item(&mut self, _ctx: &Checker_decl_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_decl_item}.
 * @param ctx the parse tree
 */
fn exit_checker_decl_item(&mut self, _ctx: &Checker_decl_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_declaration}.
 * @param ctx the parse tree
 */
fn enter_class_declaration(&mut self, _ctx: &Class_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_declaration}.
 * @param ctx the parse tree
 */
fn exit_class_declaration(&mut self, _ctx: &Class_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_name}.
 * @param ctx the parse tree
 */
fn enter_class_name(&mut self, _ctx: &Class_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_name}.
 * @param ctx the parse tree
 */
fn exit_class_name(&mut self, _ctx: &Class_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_extension}.
 * @param ctx the parse tree
 */
fn enter_class_extension(&mut self, _ctx: &Class_extensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_extension}.
 * @param ctx the parse tree
 */
fn exit_class_extension(&mut self, _ctx: &Class_extensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_implementation}.
 * @param ctx the parse tree
 */
fn enter_class_implementation(&mut self, _ctx: &Class_implementationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_implementation}.
 * @param ctx the parse tree
 */
fn exit_class_implementation(&mut self, _ctx: &Class_implementationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_class_type}.
 * @param ctx the parse tree
 */
fn enter_interface_class_type(&mut self, _ctx: &Interface_class_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_class_type}.
 * @param ctx the parse tree
 */
fn exit_interface_class_type(&mut self, _ctx: &Interface_class_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_class_declaration}.
 * @param ctx the parse tree
 */
fn enter_interface_class_declaration(&mut self, _ctx: &Interface_class_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_class_declaration}.
 * @param ctx the parse tree
 */
fn exit_interface_class_declaration(&mut self, _ctx: &Interface_class_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_class_extension}.
 * @param ctx the parse tree
 */
fn enter_interface_class_extension(&mut self, _ctx: &Interface_class_extensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_class_extension}.
 * @param ctx the parse tree
 */
fn exit_interface_class_extension(&mut self, _ctx: &Interface_class_extensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_class_item}.
 * @param ctx the parse tree
 */
fn enter_interface_class_item(&mut self, _ctx: &Interface_class_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_class_item}.
 * @param ctx the parse tree
 */
fn exit_interface_class_item(&mut self, _ctx: &Interface_class_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_class_method}.
 * @param ctx the parse tree
 */
fn enter_interface_class_method(&mut self, _ctx: &Interface_class_methodContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_class_method}.
 * @param ctx the parse tree
 */
fn exit_interface_class_method(&mut self, _ctx: &Interface_class_methodContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_declaration}.
 * @param ctx the parse tree
 */
fn enter_package_declaration(&mut self, _ctx: &Package_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_declaration}.
 * @param ctx the parse tree
 */
fn exit_package_declaration(&mut self, _ctx: &Package_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_name}.
 * @param ctx the parse tree
 */
fn enter_package_name(&mut self, _ctx: &Package_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_name}.
 * @param ctx the parse tree
 */
fn exit_package_name(&mut self, _ctx: &Package_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pkg_decl_item}.
 * @param ctx the parse tree
 */
fn enter_pkg_decl_item(&mut self, _ctx: &Pkg_decl_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pkg_decl_item}.
 * @param ctx the parse tree
 */
fn exit_pkg_decl_item(&mut self, _ctx: &Pkg_decl_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timeunits_declaration}.
 * @param ctx the parse tree
 */
fn enter_timeunits_declaration(&mut self, _ctx: &Timeunits_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timeunits_declaration}.
 * @param ctx the parse tree
 */
fn exit_timeunits_declaration(&mut self, _ctx: &Timeunits_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parameter_port_list}.
 * @param ctx the parse tree
 */
fn enter_parameter_port_list(&mut self, _ctx: &Parameter_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parameter_port_list}.
 * @param ctx the parse tree
 */
fn exit_parameter_port_list(&mut self, _ctx: &Parameter_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parameter_port_declaration}.
 * @param ctx the parse tree
 */
fn enter_parameter_port_declaration(&mut self, _ctx: &Parameter_port_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parameter_port_declaration}.
 * @param ctx the parse tree
 */
fn exit_parameter_port_declaration(&mut self, _ctx: &Parameter_port_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_port_declarations}.
 * @param ctx the parse tree
 */
fn enter_list_of_port_declarations(&mut self, _ctx: &List_of_port_declarationsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_port_declarations}.
 * @param ctx the parse tree
 */
fn exit_list_of_port_declarations(&mut self, _ctx: &List_of_port_declarationsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_decl}.
 * @param ctx the parse tree
 */
fn enter_port_decl(&mut self, _ctx: &Port_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_decl}.
 * @param ctx the parse tree
 */
fn exit_port_decl(&mut self, _ctx: &Port_declContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_declaration}.
 * @param ctx the parse tree
 */
fn enter_port_declaration(&mut self, _ctx: &Port_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_declaration}.
 * @param ctx the parse tree
 */
fn exit_port_declaration(&mut self, _ctx: &Port_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port}.
 * @param ctx the parse tree
 */
fn enter_port(&mut self, _ctx: &PortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port}.
 * @param ctx the parse tree
 */
fn exit_port(&mut self, _ctx: &PortContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_implicit}.
 * @param ctx the parse tree
 */
fn enter_port_implicit(&mut self, _ctx: &Port_implicitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_implicit}.
 * @param ctx the parse tree
 */
fn exit_port_implicit(&mut self, _ctx: &Port_implicitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_expression}.
 * @param ctx the parse tree
 */
fn enter_port_expression(&mut self, _ctx: &Port_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_expression}.
 * @param ctx the parse tree
 */
fn exit_port_expression(&mut self, _ctx: &Port_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_reference}.
 * @param ctx the parse tree
 */
fn enter_port_reference(&mut self, _ctx: &Port_referenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_reference}.
 * @param ctx the parse tree
 */
fn exit_port_reference(&mut self, _ctx: &Port_referenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_direction}.
 * @param ctx the parse tree
 */
fn enter_port_direction(&mut self, _ctx: &Port_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_direction}.
 * @param ctx the parse tree
 */
fn exit_port_direction(&mut self, _ctx: &Port_directionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ansi_port_declaration}.
 * @param ctx the parse tree
 */
fn enter_ansi_port_declaration(&mut self, _ctx: &Ansi_port_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ansi_port_declaration}.
 * @param ctx the parse tree
 */
fn exit_ansi_port_declaration(&mut self, _ctx: &Ansi_port_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#elaboration_system_task}.
 * @param ctx the parse tree
 */
fn enter_elaboration_system_task(&mut self, _ctx: &Elaboration_system_taskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#elaboration_system_task}.
 * @param ctx the parse tree
 */
fn exit_elaboration_system_task(&mut self, _ctx: &Elaboration_system_taskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#fatal_arg_list}.
 * @param ctx the parse tree
 */
fn enter_fatal_arg_list(&mut self, _ctx: &Fatal_arg_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#fatal_arg_list}.
 * @param ctx the parse tree
 */
fn exit_fatal_arg_list(&mut self, _ctx: &Fatal_arg_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#finish_number}.
 * @param ctx the parse tree
 */
fn enter_finish_number(&mut self, _ctx: &Finish_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#finish_number}.
 * @param ctx the parse tree
 */
fn exit_finish_number(&mut self, _ctx: &Finish_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_common_item}.
 * @param ctx the parse tree
 */
fn enter_module_common_item(&mut self, _ctx: &Module_common_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_common_item}.
 * @param ctx the parse tree
 */
fn exit_module_common_item(&mut self, _ctx: &Module_common_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_item}.
 * @param ctx the parse tree
 */
fn enter_module_item(&mut self, _ctx: &Module_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_item}.
 * @param ctx the parse tree
 */
fn exit_module_item(&mut self, _ctx: &Module_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_item_declaration}.
 * @param ctx the parse tree
 */
fn enter_module_item_declaration(&mut self, _ctx: &Module_item_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_item_declaration}.
 * @param ctx the parse tree
 */
fn exit_module_item_declaration(&mut self, _ctx: &Module_item_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parameter_override}.
 * @param ctx the parse tree
 */
fn enter_parameter_override(&mut self, _ctx: &Parameter_overrideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parameter_override}.
 * @param ctx the parse tree
 */
fn exit_parameter_override(&mut self, _ctx: &Parameter_overrideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bind_directive}.
 * @param ctx the parse tree
 */
fn enter_bind_directive(&mut self, _ctx: &Bind_directiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bind_directive}.
 * @param ctx the parse tree
 */
fn exit_bind_directive(&mut self, _ctx: &Bind_directiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bind_target_scope}.
 * @param ctx the parse tree
 */
fn enter_bind_target_scope(&mut self, _ctx: &Bind_target_scopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bind_target_scope}.
 * @param ctx the parse tree
 */
fn exit_bind_target_scope(&mut self, _ctx: &Bind_target_scopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bind_target_instance}.
 * @param ctx the parse tree
 */
fn enter_bind_target_instance(&mut self, _ctx: &Bind_target_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bind_target_instance}.
 * @param ctx the parse tree
 */
fn exit_bind_target_instance(&mut self, _ctx: &Bind_target_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bind_target_instance_list}.
 * @param ctx the parse tree
 */
fn enter_bind_target_instance_list(&mut self, _ctx: &Bind_target_instance_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bind_target_instance_list}.
 * @param ctx the parse tree
 */
fn exit_bind_target_instance_list(&mut self, _ctx: &Bind_target_instance_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bind_instantiation}.
 * @param ctx the parse tree
 */
fn enter_bind_instantiation(&mut self, _ctx: &Bind_instantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bind_instantiation}.
 * @param ctx the parse tree
 */
fn exit_bind_instantiation(&mut self, _ctx: &Bind_instantiationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#config_declaration}.
 * @param ctx the parse tree
 */
fn enter_config_declaration(&mut self, _ctx: &Config_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#config_declaration}.
 * @param ctx the parse tree
 */
fn exit_config_declaration(&mut self, _ctx: &Config_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#config_name}.
 * @param ctx the parse tree
 */
fn enter_config_name(&mut self, _ctx: &Config_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#config_name}.
 * @param ctx the parse tree
 */
fn exit_config_name(&mut self, _ctx: &Config_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#design_statement}.
 * @param ctx the parse tree
 */
fn enter_design_statement(&mut self, _ctx: &Design_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#design_statement}.
 * @param ctx the parse tree
 */
fn exit_design_statement(&mut self, _ctx: &Design_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#design_statement_item}.
 * @param ctx the parse tree
 */
fn enter_design_statement_item(&mut self, _ctx: &Design_statement_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#design_statement_item}.
 * @param ctx the parse tree
 */
fn exit_design_statement_item(&mut self, _ctx: &Design_statement_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#config_rule_statement}.
 * @param ctx the parse tree
 */
fn enter_config_rule_statement(&mut self, _ctx: &Config_rule_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#config_rule_statement}.
 * @param ctx the parse tree
 */
fn exit_config_rule_statement(&mut self, _ctx: &Config_rule_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#default_clause}.
 * @param ctx the parse tree
 */
fn enter_default_clause(&mut self, _ctx: &Default_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#default_clause}.
 * @param ctx the parse tree
 */
fn exit_default_clause(&mut self, _ctx: &Default_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#inst_clause}.
 * @param ctx the parse tree
 */
fn enter_inst_clause(&mut self, _ctx: &Inst_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#inst_clause}.
 * @param ctx the parse tree
 */
fn exit_inst_clause(&mut self, _ctx: &Inst_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#inst_name}.
 * @param ctx the parse tree
 */
fn enter_inst_name(&mut self, _ctx: &Inst_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#inst_name}.
 * @param ctx the parse tree
 */
fn exit_inst_name(&mut self, _ctx: &Inst_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cell_clause}.
 * @param ctx the parse tree
 */
fn enter_cell_clause(&mut self, _ctx: &Cell_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cell_clause}.
 * @param ctx the parse tree
 */
fn exit_cell_clause(&mut self, _ctx: &Cell_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#liblist_clause}.
 * @param ctx the parse tree
 */
fn enter_liblist_clause(&mut self, _ctx: &Liblist_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#liblist_clause}.
 * @param ctx the parse tree
 */
fn exit_liblist_clause(&mut self, _ctx: &Liblist_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#use_clause}.
 * @param ctx the parse tree
 */
fn enter_use_clause(&mut self, _ctx: &Use_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#use_clause}.
 * @param ctx the parse tree
 */
fn exit_use_clause(&mut self, _ctx: &Use_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#extern_tf_declaration}.
 * @param ctx the parse tree
 */
fn enter_extern_tf_declaration(&mut self, _ctx: &Extern_tf_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#extern_tf_declaration}.
 * @param ctx the parse tree
 */
fn exit_extern_tf_declaration(&mut self, _ctx: &Extern_tf_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_item}.
 * @param ctx the parse tree
 */
fn enter_interface_item(&mut self, _ctx: &Interface_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_item}.
 * @param ctx the parse tree
 */
fn exit_interface_item(&mut self, _ctx: &Interface_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#program_item}.
 * @param ctx the parse tree
 */
fn enter_program_item(&mut self, _ctx: &Program_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#program_item}.
 * @param ctx the parse tree
 */
fn exit_program_item(&mut self, _ctx: &Program_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_port_list}.
 * @param ctx the parse tree
 */
fn enter_checker_port_list(&mut self, _ctx: &Checker_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_port_list}.
 * @param ctx the parse tree
 */
fn exit_checker_port_list(&mut self, _ctx: &Checker_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_port_item}.
 * @param ctx the parse tree
 */
fn enter_checker_port_item(&mut self, _ctx: &Checker_port_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_port_item}.
 * @param ctx the parse tree
 */
fn exit_checker_port_item(&mut self, _ctx: &Checker_port_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_port_direction}.
 * @param ctx the parse tree
 */
fn enter_checker_port_direction(&mut self, _ctx: &Checker_port_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_port_direction}.
 * @param ctx the parse tree
 */
fn exit_checker_port_direction(&mut self, _ctx: &Checker_port_directionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_item}.
 * @param ctx the parse tree
 */
fn enter_checker_item(&mut self, _ctx: &Checker_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_item}.
 * @param ctx the parse tree
 */
fn exit_checker_item(&mut self, _ctx: &Checker_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_item_declaration}.
 * @param ctx the parse tree
 */
fn enter_checker_item_declaration(&mut self, _ctx: &Checker_item_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_item_declaration}.
 * @param ctx the parse tree
 */
fn exit_checker_item_declaration(&mut self, _ctx: &Checker_item_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_item}.
 * @param ctx the parse tree
 */
fn enter_class_item(&mut self, _ctx: &Class_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_item}.
 * @param ctx the parse tree
 */
fn exit_class_item(&mut self, _ctx: &Class_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_property}.
 * @param ctx the parse tree
 */
fn enter_class_property(&mut self, _ctx: &Class_propertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_property}.
 * @param ctx the parse tree
 */
fn exit_class_property(&mut self, _ctx: &Class_propertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_method}.
 * @param ctx the parse tree
 */
fn enter_class_method(&mut self, _ctx: &Class_methodContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_method}.
 * @param ctx the parse tree
 */
fn exit_class_method(&mut self, _ctx: &Class_methodContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_constructor_prototype}.
 * @param ctx the parse tree
 */
fn enter_class_constructor_prototype(&mut self, _ctx: &Class_constructor_prototypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_constructor_prototype}.
 * @param ctx the parse tree
 */
fn exit_class_constructor_prototype(&mut self, _ctx: &Class_constructor_prototypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_list}.
 * @param ctx the parse tree
 */
fn enter_port_list(&mut self, _ctx: &Port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_list}.
 * @param ctx the parse tree
 */
fn exit_port_list(&mut self, _ctx: &Port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_constraint}.
 * @param ctx the parse tree
 */
fn enter_class_constraint(&mut self, _ctx: &Class_constraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_constraint}.
 * @param ctx the parse tree
 */
fn exit_class_constraint(&mut self, _ctx: &Class_constraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_item_qualifier}.
 * @param ctx the parse tree
 */
fn enter_class_item_qualifier(&mut self, _ctx: &Class_item_qualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_item_qualifier}.
 * @param ctx the parse tree
 */
fn exit_class_item_qualifier(&mut self, _ctx: &Class_item_qualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_qualifier}.
 * @param ctx the parse tree
 */
fn enter_property_qualifier(&mut self, _ctx: &Property_qualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_qualifier}.
 * @param ctx the parse tree
 */
fn exit_property_qualifier(&mut self, _ctx: &Property_qualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#random_qualifier}.
 * @param ctx the parse tree
 */
fn enter_random_qualifier(&mut self, _ctx: &Random_qualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#random_qualifier}.
 * @param ctx the parse tree
 */
fn exit_random_qualifier(&mut self, _ctx: &Random_qualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#method_qualifier}.
 * @param ctx the parse tree
 */
fn enter_method_qualifier(&mut self, _ctx: &Method_qualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#method_qualifier}.
 * @param ctx the parse tree
 */
fn exit_method_qualifier(&mut self, _ctx: &Method_qualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#method_prototype}.
 * @param ctx the parse tree
 */
fn enter_method_prototype(&mut self, _ctx: &Method_prototypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#method_prototype}.
 * @param ctx the parse tree
 */
fn exit_method_prototype(&mut self, _ctx: &Method_prototypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_constructor_declaration}.
 * @param ctx the parse tree
 */
fn enter_class_constructor_declaration(&mut self, _ctx: &Class_constructor_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_constructor_declaration}.
 * @param ctx the parse tree
 */
fn exit_class_constructor_declaration(&mut self, _ctx: &Class_constructor_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#super_class_constructor_call}.
 * @param ctx the parse tree
 */
fn enter_super_class_constructor_call(&mut self, _ctx: &Super_class_constructor_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#super_class_constructor_call}.
 * @param ctx the parse tree
 */
fn exit_super_class_constructor_call(&mut self, _ctx: &Super_class_constructor_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_declaration}.
 * @param ctx the parse tree
 */
fn enter_constraint_declaration(&mut self, _ctx: &Constraint_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_declaration}.
 * @param ctx the parse tree
 */
fn exit_constraint_declaration(&mut self, _ctx: &Constraint_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_block}.
 * @param ctx the parse tree
 */
fn enter_constraint_block(&mut self, _ctx: &Constraint_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_block}.
 * @param ctx the parse tree
 */
fn exit_constraint_block(&mut self, _ctx: &Constraint_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_block_item}.
 * @param ctx the parse tree
 */
fn enter_constraint_block_item(&mut self, _ctx: &Constraint_block_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_block_item}.
 * @param ctx the parse tree
 */
fn exit_constraint_block_item(&mut self, _ctx: &Constraint_block_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#solve_before_list}.
 * @param ctx the parse tree
 */
fn enter_solve_before_list(&mut self, _ctx: &Solve_before_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#solve_before_list}.
 * @param ctx the parse tree
 */
fn exit_solve_before_list(&mut self, _ctx: &Solve_before_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_primary}.
 * @param ctx the parse tree
 */
fn enter_constraint_primary(&mut self, _ctx: &Constraint_primaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_primary}.
 * @param ctx the parse tree
 */
fn exit_constraint_primary(&mut self, _ctx: &Constraint_primaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_expression}.
 * @param ctx the parse tree
 */
fn enter_constraint_expression(&mut self, _ctx: &Constraint_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_expression}.
 * @param ctx the parse tree
 */
fn exit_constraint_expression(&mut self, _ctx: &Constraint_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#uniqueness_constraint}.
 * @param ctx the parse tree
 */
fn enter_uniqueness_constraint(&mut self, _ctx: &Uniqueness_constraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#uniqueness_constraint}.
 * @param ctx the parse tree
 */
fn exit_uniqueness_constraint(&mut self, _ctx: &Uniqueness_constraintContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_set}.
 * @param ctx the parse tree
 */
fn enter_constraint_set(&mut self, _ctx: &Constraint_setContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_set}.
 * @param ctx the parse tree
 */
fn exit_constraint_set(&mut self, _ctx: &Constraint_setContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dist_list}.
 * @param ctx the parse tree
 */
fn enter_dist_list(&mut self, _ctx: &Dist_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dist_list}.
 * @param ctx the parse tree
 */
fn exit_dist_list(&mut self, _ctx: &Dist_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dist_item}.
 * @param ctx the parse tree
 */
fn enter_dist_item(&mut self, _ctx: &Dist_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dist_item}.
 * @param ctx the parse tree
 */
fn exit_dist_item(&mut self, _ctx: &Dist_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dist_weight}.
 * @param ctx the parse tree
 */
fn enter_dist_weight(&mut self, _ctx: &Dist_weightContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dist_weight}.
 * @param ctx the parse tree
 */
fn exit_dist_weight(&mut self, _ctx: &Dist_weightContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_prototype}.
 * @param ctx the parse tree
 */
fn enter_constraint_prototype(&mut self, _ctx: &Constraint_prototypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_prototype}.
 * @param ctx the parse tree
 */
fn exit_constraint_prototype(&mut self, _ctx: &Constraint_prototypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_prototype_qualifier}.
 * @param ctx the parse tree
 */
fn enter_constraint_prototype_qualifier(&mut self, _ctx: &Constraint_prototype_qualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_prototype_qualifier}.
 * @param ctx the parse tree
 */
fn exit_constraint_prototype_qualifier(&mut self, _ctx: &Constraint_prototype_qualifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#extern_constraint_declaration}.
 * @param ctx the parse tree
 */
fn enter_extern_constraint_declaration(&mut self, _ctx: &Extern_constraint_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#extern_constraint_declaration}.
 * @param ctx the parse tree
 */
fn exit_extern_constraint_declaration(&mut self, _ctx: &Extern_constraint_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#identifier_list}.
 * @param ctx the parse tree
 */
fn enter_identifier_list(&mut self, _ctx: &Identifier_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#identifier_list}.
 * @param ctx the parse tree
 */
fn exit_identifier_list(&mut self, _ctx: &Identifier_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_item}.
 * @param ctx the parse tree
 */
fn enter_package_item(&mut self, _ctx: &Package_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_item}.
 * @param ctx the parse tree
 */
fn exit_package_item(&mut self, _ctx: &Package_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_item_declaration}.
 * @param ctx the parse tree
 */
fn enter_package_item_declaration(&mut self, _ctx: &Package_item_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_item_declaration}.
 * @param ctx the parse tree
 */
fn exit_package_item_declaration(&mut self, _ctx: &Package_item_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#anonymous_program}.
 * @param ctx the parse tree
 */
fn enter_anonymous_program(&mut self, _ctx: &Anonymous_programContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#anonymous_program}.
 * @param ctx the parse tree
 */
fn exit_anonymous_program(&mut self, _ctx: &Anonymous_programContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#anonymous_program_item}.
 * @param ctx the parse tree
 */
fn enter_anonymous_program_item(&mut self, _ctx: &Anonymous_program_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#anonymous_program_item}.
 * @param ctx the parse tree
 */
fn exit_anonymous_program_item(&mut self, _ctx: &Anonymous_program_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#local_parameter_declaration}.
 * @param ctx the parse tree
 */
fn enter_local_parameter_declaration(&mut self, _ctx: &Local_parameter_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#local_parameter_declaration}.
 * @param ctx the parse tree
 */
fn exit_local_parameter_declaration(&mut self, _ctx: &Local_parameter_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parameter_declaration}.
 * @param ctx the parse tree
 */
fn enter_parameter_declaration(&mut self, _ctx: &Parameter_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parameter_declaration}.
 * @param ctx the parse tree
 */
fn exit_parameter_declaration(&mut self, _ctx: &Parameter_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specparam_declaration}.
 * @param ctx the parse tree
 */
fn enter_specparam_declaration(&mut self, _ctx: &Specparam_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specparam_declaration}.
 * @param ctx the parse tree
 */
fn exit_specparam_declaration(&mut self, _ctx: &Specparam_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#inout_declaration}.
 * @param ctx the parse tree
 */
fn enter_inout_declaration(&mut self, _ctx: &Inout_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#inout_declaration}.
 * @param ctx the parse tree
 */
fn exit_inout_declaration(&mut self, _ctx: &Inout_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#input_declaration}.
 * @param ctx the parse tree
 */
fn enter_input_declaration(&mut self, _ctx: &Input_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#input_declaration}.
 * @param ctx the parse tree
 */
fn exit_input_declaration(&mut self, _ctx: &Input_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#output_declaration}.
 * @param ctx the parse tree
 */
fn enter_output_declaration(&mut self, _ctx: &Output_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#output_declaration}.
 * @param ctx the parse tree
 */
fn exit_output_declaration(&mut self, _ctx: &Output_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_port_declaration}.
 * @param ctx the parse tree
 */
fn enter_interface_port_declaration(&mut self, _ctx: &Interface_port_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_port_declaration}.
 * @param ctx the parse tree
 */
fn exit_interface_port_declaration(&mut self, _ctx: &Interface_port_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ref_declaration}.
 * @param ctx the parse tree
 */
fn enter_ref_declaration(&mut self, _ctx: &Ref_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ref_declaration}.
 * @param ctx the parse tree
 */
fn exit_ref_declaration(&mut self, _ctx: &Ref_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#data_declaration}.
 * @param ctx the parse tree
 */
fn enter_data_declaration(&mut self, _ctx: &Data_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#data_declaration}.
 * @param ctx the parse tree
 */
fn exit_data_declaration(&mut self, _ctx: &Data_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_import_declaration}.
 * @param ctx the parse tree
 */
fn enter_package_import_declaration(&mut self, _ctx: &Package_import_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_import_declaration}.
 * @param ctx the parse tree
 */
fn exit_package_import_declaration(&mut self, _ctx: &Package_import_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_import_item}.
 * @param ctx the parse tree
 */
fn enter_package_import_item(&mut self, _ctx: &Package_import_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_import_item}.
 * @param ctx the parse tree
 */
fn exit_package_import_item(&mut self, _ctx: &Package_import_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_export_declaration}.
 * @param ctx the parse tree
 */
fn enter_package_export_declaration(&mut self, _ctx: &Package_export_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_export_declaration}.
 * @param ctx the parse tree
 */
fn exit_package_export_declaration(&mut self, _ctx: &Package_export_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#genvar_declaration}.
 * @param ctx the parse tree
 */
fn enter_genvar_declaration(&mut self, _ctx: &Genvar_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#genvar_declaration}.
 * @param ctx the parse tree
 */
fn exit_genvar_declaration(&mut self, _ctx: &Genvar_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_declaration}.
 * @param ctx the parse tree
 */
fn enter_net_declaration(&mut self, _ctx: &Net_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_declaration}.
 * @param ctx the parse tree
 */
fn exit_net_declaration(&mut self, _ctx: &Net_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_id}.
 * @param ctx the parse tree
 */
fn enter_net_id(&mut self, _ctx: &Net_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_id}.
 * @param ctx the parse tree
 */
fn exit_net_id(&mut self, _ctx: &Net_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#type_declaration}.
 * @param ctx the parse tree
 */
fn enter_type_declaration(&mut self, _ctx: &Type_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#type_declaration}.
 * @param ctx the parse tree
 */
fn exit_type_declaration(&mut self, _ctx: &Type_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_type_declaration}.
 * @param ctx the parse tree
 */
fn enter_net_type_declaration(&mut self, _ctx: &Net_type_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_type_declaration}.
 * @param ctx the parse tree
 */
fn exit_net_type_declaration(&mut self, _ctx: &Net_type_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_type_decl_with}.
 * @param ctx the parse tree
 */
fn enter_net_type_decl_with(&mut self, _ctx: &Net_type_decl_withContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_type_decl_with}.
 * @param ctx the parse tree
 */
fn exit_net_type_decl_with(&mut self, _ctx: &Net_type_decl_withContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#lifetime}.
 * @param ctx the parse tree
 */
fn enter_lifetime(&mut self, _ctx: &LifetimeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#lifetime}.
 * @param ctx the parse tree
 */
fn exit_lifetime(&mut self, _ctx: &LifetimeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#data_type}.
 * @param ctx the parse tree
 */
fn enter_data_type(&mut self, _ctx: &Data_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#data_type}.
 * @param ctx the parse tree
 */
fn exit_data_type(&mut self, _ctx: &Data_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#data_type_or_implicit}.
 * @param ctx the parse tree
 */
fn enter_data_type_or_implicit(&mut self, _ctx: &Data_type_or_implicitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#data_type_or_implicit}.
 * @param ctx the parse tree
 */
fn exit_data_type_or_implicit(&mut self, _ctx: &Data_type_or_implicitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#implicit_data_type}.
 * @param ctx the parse tree
 */
fn enter_implicit_data_type(&mut self, _ctx: &Implicit_data_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#implicit_data_type}.
 * @param ctx the parse tree
 */
fn exit_implicit_data_type(&mut self, _ctx: &Implicit_data_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enum_base_type}.
 * @param ctx the parse tree
 */
fn enter_enum_base_type(&mut self, _ctx: &Enum_base_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enum_base_type}.
 * @param ctx the parse tree
 */
fn exit_enum_base_type(&mut self, _ctx: &Enum_base_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enum_name_declaration}.
 * @param ctx the parse tree
 */
fn enter_enum_name_declaration(&mut self, _ctx: &Enum_name_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enum_name_declaration}.
 * @param ctx the parse tree
 */
fn exit_enum_name_declaration(&mut self, _ctx: &Enum_name_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enum_name_suffix_range}.
 * @param ctx the parse tree
 */
fn enter_enum_name_suffix_range(&mut self, _ctx: &Enum_name_suffix_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enum_name_suffix_range}.
 * @param ctx the parse tree
 */
fn exit_enum_name_suffix_range(&mut self, _ctx: &Enum_name_suffix_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_scope}.
 * @param ctx the parse tree
 */
fn enter_class_scope(&mut self, _ctx: &Class_scopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_scope}.
 * @param ctx the parse tree
 */
fn exit_class_scope(&mut self, _ctx: &Class_scopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_type}.
 * @param ctx the parse tree
 */
fn enter_class_type(&mut self, _ctx: &Class_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_type}.
 * @param ctx the parse tree
 */
fn exit_class_type(&mut self, _ctx: &Class_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_ref}.
 * @param ctx the parse tree
 */
fn enter_class_ref(&mut self, _ctx: &Class_refContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_ref}.
 * @param ctx the parse tree
 */
fn exit_class_ref(&mut self, _ctx: &Class_refContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_or_class_scope}.
 * @param ctx the parse tree
 */
fn enter_package_or_class_scope(&mut self, _ctx: &Package_or_class_scopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_or_class_scope}.
 * @param ctx the parse tree
 */
fn exit_package_or_class_scope(&mut self, _ctx: &Package_or_class_scopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#integer_type}.
 * @param ctx the parse tree
 */
fn enter_integer_type(&mut self, _ctx: &Integer_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#integer_type}.
 * @param ctx the parse tree
 */
fn exit_integer_type(&mut self, _ctx: &Integer_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#integer_atom_type}.
 * @param ctx the parse tree
 */
fn enter_integer_atom_type(&mut self, _ctx: &Integer_atom_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#integer_atom_type}.
 * @param ctx the parse tree
 */
fn exit_integer_atom_type(&mut self, _ctx: &Integer_atom_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#integer_vector_type}.
 * @param ctx the parse tree
 */
fn enter_integer_vector_type(&mut self, _ctx: &Integer_vector_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#integer_vector_type}.
 * @param ctx the parse tree
 */
fn exit_integer_vector_type(&mut self, _ctx: &Integer_vector_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#non_integer_type}.
 * @param ctx the parse tree
 */
fn enter_non_integer_type(&mut self, _ctx: &Non_integer_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#non_integer_type}.
 * @param ctx the parse tree
 */
fn exit_non_integer_type(&mut self, _ctx: &Non_integer_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_type}.
 * @param ctx the parse tree
 */
fn enter_net_type(&mut self, _ctx: &Net_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_type}.
 * @param ctx the parse tree
 */
fn exit_net_type(&mut self, _ctx: &Net_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_port_type}.
 * @param ctx the parse tree
 */
fn enter_net_port_type(&mut self, _ctx: &Net_port_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_port_type}.
 * @param ctx the parse tree
 */
fn exit_net_port_type(&mut self, _ctx: &Net_port_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_port_type}.
 * @param ctx the parse tree
 */
fn enter_variable_port_type(&mut self, _ctx: &Variable_port_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_port_type}.
 * @param ctx the parse tree
 */
fn exit_variable_port_type(&mut self, _ctx: &Variable_port_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#var_data_type}.
 * @param ctx the parse tree
 */
fn enter_var_data_type(&mut self, _ctx: &Var_data_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#var_data_type}.
 * @param ctx the parse tree
 */
fn exit_var_data_type(&mut self, _ctx: &Var_data_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#signing}.
 * @param ctx the parse tree
 */
fn enter_signing(&mut self, _ctx: &SigningContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#signing}.
 * @param ctx the parse tree
 */
fn exit_signing(&mut self, _ctx: &SigningContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_type}.
 * @param ctx the parse tree
 */
fn enter_simple_type(&mut self, _ctx: &Simple_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_type}.
 * @param ctx the parse tree
 */
fn exit_simple_type(&mut self, _ctx: &Simple_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#struct_union_member}.
 * @param ctx the parse tree
 */
fn enter_struct_union_member(&mut self, _ctx: &Struct_union_memberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#struct_union_member}.
 * @param ctx the parse tree
 */
fn exit_struct_union_member(&mut self, _ctx: &Struct_union_memberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#data_type_or_void}.
 * @param ctx the parse tree
 */
fn enter_data_type_or_void(&mut self, _ctx: &Data_type_or_voidContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#data_type_or_void}.
 * @param ctx the parse tree
 */
fn exit_data_type_or_void(&mut self, _ctx: &Data_type_or_voidContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#struct_union}.
 * @param ctx the parse tree
 */
fn enter_struct_union(&mut self, _ctx: &Struct_unionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#struct_union}.
 * @param ctx the parse tree
 */
fn exit_struct_union(&mut self, _ctx: &Struct_unionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#type_reference}.
 * @param ctx the parse tree
 */
fn enter_type_reference(&mut self, _ctx: &Type_referenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#type_reference}.
 * @param ctx the parse tree
 */
fn exit_type_reference(&mut self, _ctx: &Type_referenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#drive_strength}.
 * @param ctx the parse tree
 */
fn enter_drive_strength(&mut self, _ctx: &Drive_strengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#drive_strength}.
 * @param ctx the parse tree
 */
fn exit_drive_strength(&mut self, _ctx: &Drive_strengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#strength0}.
 * @param ctx the parse tree
 */
fn enter_strength0(&mut self, _ctx: &Strength0Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#strength0}.
 * @param ctx the parse tree
 */
fn exit_strength0(&mut self, _ctx: &Strength0Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#strength1}.
 * @param ctx the parse tree
 */
fn enter_strength1(&mut self, _ctx: &Strength1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#strength1}.
 * @param ctx the parse tree
 */
fn exit_strength1(&mut self, _ctx: &Strength1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#charge_strength}.
 * @param ctx the parse tree
 */
fn enter_charge_strength(&mut self, _ctx: &Charge_strengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#charge_strength}.
 * @param ctx the parse tree
 */
fn exit_charge_strength(&mut self, _ctx: &Charge_strengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delay3}.
 * @param ctx the parse tree
 */
fn enter_delay3(&mut self, _ctx: &Delay3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delay3}.
 * @param ctx the parse tree
 */
fn exit_delay3(&mut self, _ctx: &Delay3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delay2}.
 * @param ctx the parse tree
 */
fn enter_delay2(&mut self, _ctx: &Delay2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delay2}.
 * @param ctx the parse tree
 */
fn exit_delay2(&mut self, _ctx: &Delay2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delay_value}.
 * @param ctx the parse tree
 */
fn enter_delay_value(&mut self, _ctx: &Delay_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delay_value}.
 * @param ctx the parse tree
 */
fn exit_delay_value(&mut self, _ctx: &Delay_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_defparam_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_defparam_assignments(&mut self, _ctx: &List_of_defparam_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_defparam_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_defparam_assignments(&mut self, _ctx: &List_of_defparam_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_genvar_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_genvar_identifiers(&mut self, _ctx: &List_of_genvar_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_genvar_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_genvar_identifiers(&mut self, _ctx: &List_of_genvar_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_interface_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_interface_identifiers(&mut self, _ctx: &List_of_interface_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_interface_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_interface_identifiers(&mut self, _ctx: &List_of_interface_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_id}.
 * @param ctx the parse tree
 */
fn enter_interface_id(&mut self, _ctx: &Interface_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_id}.
 * @param ctx the parse tree
 */
fn exit_interface_id(&mut self, _ctx: &Interface_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_net_decl_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_net_decl_assignments(&mut self, _ctx: &List_of_net_decl_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_net_decl_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_net_decl_assignments(&mut self, _ctx: &List_of_net_decl_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_param_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_param_assignments(&mut self, _ctx: &List_of_param_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_param_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_param_assignments(&mut self, _ctx: &List_of_param_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_port_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_port_identifiers(&mut self, _ctx: &List_of_port_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_port_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_port_identifiers(&mut self, _ctx: &List_of_port_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_id}.
 * @param ctx the parse tree
 */
fn enter_port_id(&mut self, _ctx: &Port_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_id}.
 * @param ctx the parse tree
 */
fn exit_port_id(&mut self, _ctx: &Port_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_udp_port_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_udp_port_identifiers(&mut self, _ctx: &List_of_udp_port_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_udp_port_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_udp_port_identifiers(&mut self, _ctx: &List_of_udp_port_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_specparam_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_specparam_assignments(&mut self, _ctx: &List_of_specparam_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_specparam_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_specparam_assignments(&mut self, _ctx: &List_of_specparam_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_tf_variable_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_tf_variable_identifiers(&mut self, _ctx: &List_of_tf_variable_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_tf_variable_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_tf_variable_identifiers(&mut self, _ctx: &List_of_tf_variable_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_var_id}.
 * @param ctx the parse tree
 */
fn enter_tf_var_id(&mut self, _ctx: &Tf_var_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_var_id}.
 * @param ctx the parse tree
 */
fn exit_tf_var_id(&mut self, _ctx: &Tf_var_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_type_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_type_assignments(&mut self, _ctx: &List_of_type_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_type_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_type_assignments(&mut self, _ctx: &List_of_type_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_variable_decl_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_variable_decl_assignments(&mut self, _ctx: &List_of_variable_decl_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_variable_decl_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_variable_decl_assignments(&mut self, _ctx: &List_of_variable_decl_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_variable_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_variable_identifiers(&mut self, _ctx: &List_of_variable_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_variable_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_variable_identifiers(&mut self, _ctx: &List_of_variable_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#var_id}.
 * @param ctx the parse tree
 */
fn enter_var_id(&mut self, _ctx: &Var_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#var_id}.
 * @param ctx the parse tree
 */
fn exit_var_id(&mut self, _ctx: &Var_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_variable_port_identifiers}.
 * @param ctx the parse tree
 */
fn enter_list_of_variable_port_identifiers(&mut self, _ctx: &List_of_variable_port_identifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_variable_port_identifiers}.
 * @param ctx the parse tree
 */
fn exit_list_of_variable_port_identifiers(&mut self, _ctx: &List_of_variable_port_identifiersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#var_port_id}.
 * @param ctx the parse tree
 */
fn enter_var_port_id(&mut self, _ctx: &Var_port_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#var_port_id}.
 * @param ctx the parse tree
 */
fn exit_var_port_id(&mut self, _ctx: &Var_port_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#defparam_assignment}.
 * @param ctx the parse tree
 */
fn enter_defparam_assignment(&mut self, _ctx: &Defparam_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#defparam_assignment}.
 * @param ctx the parse tree
 */
fn exit_defparam_assignment(&mut self, _ctx: &Defparam_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_decl_assignment}.
 * @param ctx the parse tree
 */
fn enter_net_decl_assignment(&mut self, _ctx: &Net_decl_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_decl_assignment}.
 * @param ctx the parse tree
 */
fn exit_net_decl_assignment(&mut self, _ctx: &Net_decl_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#param_assignment}.
 * @param ctx the parse tree
 */
fn enter_param_assignment(&mut self, _ctx: &Param_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#param_assignment}.
 * @param ctx the parse tree
 */
fn exit_param_assignment(&mut self, _ctx: &Param_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specparam_assignment}.
 * @param ctx the parse tree
 */
fn enter_specparam_assignment(&mut self, _ctx: &Specparam_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specparam_assignment}.
 * @param ctx the parse tree
 */
fn exit_specparam_assignment(&mut self, _ctx: &Specparam_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#type_assignment}.
 * @param ctx the parse tree
 */
fn enter_type_assignment(&mut self, _ctx: &Type_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#type_assignment}.
 * @param ctx the parse tree
 */
fn exit_type_assignment(&mut self, _ctx: &Type_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pulse_control_specparam}.
 * @param ctx the parse tree
 */
fn enter_pulse_control_specparam(&mut self, _ctx: &Pulse_control_specparamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pulse_control_specparam}.
 * @param ctx the parse tree
 */
fn exit_pulse_control_specparam(&mut self, _ctx: &Pulse_control_specparamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#error_limit_value}.
 * @param ctx the parse tree
 */
fn enter_error_limit_value(&mut self, _ctx: &Error_limit_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#error_limit_value}.
 * @param ctx the parse tree
 */
fn exit_error_limit_value(&mut self, _ctx: &Error_limit_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#reject_limit_value}.
 * @param ctx the parse tree
 */
fn enter_reject_limit_value(&mut self, _ctx: &Reject_limit_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#reject_limit_value}.
 * @param ctx the parse tree
 */
fn exit_reject_limit_value(&mut self, _ctx: &Reject_limit_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#limit_value}.
 * @param ctx the parse tree
 */
fn enter_limit_value(&mut self, _ctx: &Limit_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#limit_value}.
 * @param ctx the parse tree
 */
fn exit_limit_value(&mut self, _ctx: &Limit_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_decl_assignment}.
 * @param ctx the parse tree
 */
fn enter_variable_decl_assignment(&mut self, _ctx: &Variable_decl_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_decl_assignment}.
 * @param ctx the parse tree
 */
fn exit_variable_decl_assignment(&mut self, _ctx: &Variable_decl_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_new}.
 * @param ctx the parse tree
 */
fn enter_class_new(&mut self, _ctx: &Class_newContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_new}.
 * @param ctx the parse tree
 */
fn exit_class_new(&mut self, _ctx: &Class_newContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dynamic_array_new}.
 * @param ctx the parse tree
 */
fn enter_dynamic_array_new(&mut self, _ctx: &Dynamic_array_newContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dynamic_array_new}.
 * @param ctx the parse tree
 */
fn exit_dynamic_array_new(&mut self, _ctx: &Dynamic_array_newContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unpacked_dimension}.
 * @param ctx the parse tree
 */
fn enter_unpacked_dimension(&mut self, _ctx: &Unpacked_dimensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unpacked_dimension}.
 * @param ctx the parse tree
 */
fn exit_unpacked_dimension(&mut self, _ctx: &Unpacked_dimensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#packed_dimension}.
 * @param ctx the parse tree
 */
fn enter_packed_dimension(&mut self, _ctx: &Packed_dimensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#packed_dimension}.
 * @param ctx the parse tree
 */
fn exit_packed_dimension(&mut self, _ctx: &Packed_dimensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#associative_dimension}.
 * @param ctx the parse tree
 */
fn enter_associative_dimension(&mut self, _ctx: &Associative_dimensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#associative_dimension}.
 * @param ctx the parse tree
 */
fn exit_associative_dimension(&mut self, _ctx: &Associative_dimensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_dimension}.
 * @param ctx the parse tree
 */
fn enter_variable_dimension(&mut self, _ctx: &Variable_dimensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_dimension}.
 * @param ctx the parse tree
 */
fn exit_variable_dimension(&mut self, _ctx: &Variable_dimensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#queue_dimension}.
 * @param ctx the parse tree
 */
fn enter_queue_dimension(&mut self, _ctx: &Queue_dimensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#queue_dimension}.
 * @param ctx the parse tree
 */
fn exit_queue_dimension(&mut self, _ctx: &Queue_dimensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unsized_dimension}.
 * @param ctx the parse tree
 */
fn enter_unsized_dimension(&mut self, _ctx: &Unsized_dimensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unsized_dimension}.
 * @param ctx the parse tree
 */
fn exit_unsized_dimension(&mut self, _ctx: &Unsized_dimensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_data_type_or_implicit}.
 * @param ctx the parse tree
 */
fn enter_function_data_type_or_implicit(&mut self, _ctx: &Function_data_type_or_implicitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_data_type_or_implicit}.
 * @param ctx the parse tree
 */
fn exit_function_data_type_or_implicit(&mut self, _ctx: &Function_data_type_or_implicitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_declaration}.
 * @param ctx the parse tree
 */
fn enter_function_declaration(&mut self, _ctx: &Function_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_declaration}.
 * @param ctx the parse tree
 */
fn exit_function_declaration(&mut self, _ctx: &Function_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_body_declaration}.
 * @param ctx the parse tree
 */
fn enter_function_body_declaration(&mut self, _ctx: &Function_body_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_body_declaration}.
 * @param ctx the parse tree
 */
fn exit_function_body_declaration(&mut self, _ctx: &Function_body_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_name}.
 * @param ctx the parse tree
 */
fn enter_function_name(&mut self, _ctx: &Function_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_name}.
 * @param ctx the parse tree
 */
fn exit_function_name(&mut self, _ctx: &Function_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_prototype}.
 * @param ctx the parse tree
 */
fn enter_function_prototype(&mut self, _ctx: &Function_prototypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_prototype}.
 * @param ctx the parse tree
 */
fn exit_function_prototype(&mut self, _ctx: &Function_prototypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dpi_import_export}.
 * @param ctx the parse tree
 */
fn enter_dpi_import_export(&mut self, _ctx: &Dpi_import_exportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dpi_import_export}.
 * @param ctx the parse tree
 */
fn exit_dpi_import_export(&mut self, _ctx: &Dpi_import_exportContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dpi_spec_string}.
 * @param ctx the parse tree
 */
fn enter_dpi_spec_string(&mut self, _ctx: &Dpi_spec_stringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dpi_spec_string}.
 * @param ctx the parse tree
 */
fn exit_dpi_spec_string(&mut self, _ctx: &Dpi_spec_stringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dpi_function_import_property}.
 * @param ctx the parse tree
 */
fn enter_dpi_function_import_property(&mut self, _ctx: &Dpi_function_import_propertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dpi_function_import_property}.
 * @param ctx the parse tree
 */
fn exit_dpi_function_import_property(&mut self, _ctx: &Dpi_function_import_propertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dpi_task_import_property}.
 * @param ctx the parse tree
 */
fn enter_dpi_task_import_property(&mut self, _ctx: &Dpi_task_import_propertyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dpi_task_import_property}.
 * @param ctx the parse tree
 */
fn exit_dpi_task_import_property(&mut self, _ctx: &Dpi_task_import_propertyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dpi_function_proto}.
 * @param ctx the parse tree
 */
fn enter_dpi_function_proto(&mut self, _ctx: &Dpi_function_protoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dpi_function_proto}.
 * @param ctx the parse tree
 */
fn exit_dpi_function_proto(&mut self, _ctx: &Dpi_function_protoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dpi_task_proto}.
 * @param ctx the parse tree
 */
fn enter_dpi_task_proto(&mut self, _ctx: &Dpi_task_protoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dpi_task_proto}.
 * @param ctx the parse tree
 */
fn exit_dpi_task_proto(&mut self, _ctx: &Dpi_task_protoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#task_declaration}.
 * @param ctx the parse tree
 */
fn enter_task_declaration(&mut self, _ctx: &Task_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#task_declaration}.
 * @param ctx the parse tree
 */
fn exit_task_declaration(&mut self, _ctx: &Task_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#task_body_declaration}.
 * @param ctx the parse tree
 */
fn enter_task_body_declaration(&mut self, _ctx: &Task_body_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#task_body_declaration}.
 * @param ctx the parse tree
 */
fn exit_task_body_declaration(&mut self, _ctx: &Task_body_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#task_name}.
 * @param ctx the parse tree
 */
fn enter_task_name(&mut self, _ctx: &Task_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#task_name}.
 * @param ctx the parse tree
 */
fn exit_task_name(&mut self, _ctx: &Task_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_item_declaration}.
 * @param ctx the parse tree
 */
fn enter_tf_item_declaration(&mut self, _ctx: &Tf_item_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_item_declaration}.
 * @param ctx the parse tree
 */
fn exit_tf_item_declaration(&mut self, _ctx: &Tf_item_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_port_list}.
 * @param ctx the parse tree
 */
fn enter_tf_port_list(&mut self, _ctx: &Tf_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_port_list}.
 * @param ctx the parse tree
 */
fn exit_tf_port_list(&mut self, _ctx: &Tf_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_port_item}.
 * @param ctx the parse tree
 */
fn enter_tf_port_item(&mut self, _ctx: &Tf_port_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_port_item}.
 * @param ctx the parse tree
 */
fn exit_tf_port_item(&mut self, _ctx: &Tf_port_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_port_id}.
 * @param ctx the parse tree
 */
fn enter_tf_port_id(&mut self, _ctx: &Tf_port_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_port_id}.
 * @param ctx the parse tree
 */
fn exit_tf_port_id(&mut self, _ctx: &Tf_port_idContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_port_direction}.
 * @param ctx the parse tree
 */
fn enter_tf_port_direction(&mut self, _ctx: &Tf_port_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_port_direction}.
 * @param ctx the parse tree
 */
fn exit_tf_port_direction(&mut self, _ctx: &Tf_port_directionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_port_declaration}.
 * @param ctx the parse tree
 */
fn enter_tf_port_declaration(&mut self, _ctx: &Tf_port_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_port_declaration}.
 * @param ctx the parse tree
 */
fn exit_tf_port_declaration(&mut self, _ctx: &Tf_port_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#task_prototype}.
 * @param ctx the parse tree
 */
fn enter_task_prototype(&mut self, _ctx: &Task_prototypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#task_prototype}.
 * @param ctx the parse tree
 */
fn exit_task_prototype(&mut self, _ctx: &Task_prototypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#block_item_declaration}.
 * @param ctx the parse tree
 */
fn enter_block_item_declaration(&mut self, _ctx: &Block_item_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#block_item_declaration}.
 * @param ctx the parse tree
 */
fn exit_block_item_declaration(&mut self, _ctx: &Block_item_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_declaration}.
 * @param ctx the parse tree
 */
fn enter_modport_declaration(&mut self, _ctx: &Modport_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_declaration}.
 * @param ctx the parse tree
 */
fn exit_modport_declaration(&mut self, _ctx: &Modport_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_item}.
 * @param ctx the parse tree
 */
fn enter_modport_item(&mut self, _ctx: &Modport_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_item}.
 * @param ctx the parse tree
 */
fn exit_modport_item(&mut self, _ctx: &Modport_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_ports_declaration}.
 * @param ctx the parse tree
 */
fn enter_modport_ports_declaration(&mut self, _ctx: &Modport_ports_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_ports_declaration}.
 * @param ctx the parse tree
 */
fn exit_modport_ports_declaration(&mut self, _ctx: &Modport_ports_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_clocking_declaration}.
 * @param ctx the parse tree
 */
fn enter_modport_clocking_declaration(&mut self, _ctx: &Modport_clocking_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_clocking_declaration}.
 * @param ctx the parse tree
 */
fn exit_modport_clocking_declaration(&mut self, _ctx: &Modport_clocking_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_simple_ports_declaration}.
 * @param ctx the parse tree
 */
fn enter_modport_simple_ports_declaration(&mut self, _ctx: &Modport_simple_ports_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_simple_ports_declaration}.
 * @param ctx the parse tree
 */
fn exit_modport_simple_ports_declaration(&mut self, _ctx: &Modport_simple_ports_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_simple_port}.
 * @param ctx the parse tree
 */
fn enter_modport_simple_port(&mut self, _ctx: &Modport_simple_portContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_simple_port}.
 * @param ctx the parse tree
 */
fn exit_modport_simple_port(&mut self, _ctx: &Modport_simple_portContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_tf_ports_declaration}.
 * @param ctx the parse tree
 */
fn enter_modport_tf_ports_declaration(&mut self, _ctx: &Modport_tf_ports_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_tf_ports_declaration}.
 * @param ctx the parse tree
 */
fn exit_modport_tf_ports_declaration(&mut self, _ctx: &Modport_tf_ports_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_tf_port}.
 * @param ctx the parse tree
 */
fn enter_modport_tf_port(&mut self, _ctx: &Modport_tf_portContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_tf_port}.
 * @param ctx the parse tree
 */
fn exit_modport_tf_port(&mut self, _ctx: &Modport_tf_portContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#import_export}.
 * @param ctx the parse tree
 */
fn enter_import_export(&mut self, _ctx: &Import_exportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#import_export}.
 * @param ctx the parse tree
 */
fn exit_import_export(&mut self, _ctx: &Import_exportContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#concurrent_assertion_item}.
 * @param ctx the parse tree
 */
fn enter_concurrent_assertion_item(&mut self, _ctx: &Concurrent_assertion_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#concurrent_assertion_item}.
 * @param ctx the parse tree
 */
fn exit_concurrent_assertion_item(&mut self, _ctx: &Concurrent_assertion_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#block_label}.
 * @param ctx the parse tree
 */
fn enter_block_label(&mut self, _ctx: &Block_labelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#block_label}.
 * @param ctx the parse tree
 */
fn exit_block_label(&mut self, _ctx: &Block_labelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#concurrent_assertion_statement}.
 * @param ctx the parse tree
 */
fn enter_concurrent_assertion_statement(&mut self, _ctx: &Concurrent_assertion_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#concurrent_assertion_statement}.
 * @param ctx the parse tree
 */
fn exit_concurrent_assertion_statement(&mut self, _ctx: &Concurrent_assertion_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assert_property_statement}.
 * @param ctx the parse tree
 */
fn enter_assert_property_statement(&mut self, _ctx: &Assert_property_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assert_property_statement}.
 * @param ctx the parse tree
 */
fn exit_assert_property_statement(&mut self, _ctx: &Assert_property_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assume_property_statement}.
 * @param ctx the parse tree
 */
fn enter_assume_property_statement(&mut self, _ctx: &Assume_property_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assume_property_statement}.
 * @param ctx the parse tree
 */
fn exit_assume_property_statement(&mut self, _ctx: &Assume_property_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cover_property_statement}.
 * @param ctx the parse tree
 */
fn enter_cover_property_statement(&mut self, _ctx: &Cover_property_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cover_property_statement}.
 * @param ctx the parse tree
 */
fn exit_cover_property_statement(&mut self, _ctx: &Cover_property_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#expect_property_statement}.
 * @param ctx the parse tree
 */
fn enter_expect_property_statement(&mut self, _ctx: &Expect_property_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#expect_property_statement}.
 * @param ctx the parse tree
 */
fn exit_expect_property_statement(&mut self, _ctx: &Expect_property_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cover_sequence_statement}.
 * @param ctx the parse tree
 */
fn enter_cover_sequence_statement(&mut self, _ctx: &Cover_sequence_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cover_sequence_statement}.
 * @param ctx the parse tree
 */
fn exit_cover_sequence_statement(&mut self, _ctx: &Cover_sequence_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#restrict_property_statement}.
 * @param ctx the parse tree
 */
fn enter_restrict_property_statement(&mut self, _ctx: &Restrict_property_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#restrict_property_statement}.
 * @param ctx the parse tree
 */
fn exit_restrict_property_statement(&mut self, _ctx: &Restrict_property_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_instance}.
 * @param ctx the parse tree
 */
fn enter_property_instance(&mut self, _ctx: &Property_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_instance}.
 * @param ctx the parse tree
 */
fn exit_property_instance(&mut self, _ctx: &Property_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#prop_arg_list}.
 * @param ctx the parse tree
 */
fn enter_prop_arg_list(&mut self, _ctx: &Prop_arg_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#prop_arg_list}.
 * @param ctx the parse tree
 */
fn exit_prop_arg_list(&mut self, _ctx: &Prop_arg_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_list_of_arguments}.
 * @param ctx the parse tree
 */
fn enter_property_list_of_arguments(&mut self, _ctx: &Property_list_of_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_list_of_arguments}.
 * @param ctx the parse tree
 */
fn exit_property_list_of_arguments(&mut self, _ctx: &Property_list_of_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#prop_ordered_arg}.
 * @param ctx the parse tree
 */
fn enter_prop_ordered_arg(&mut self, _ctx: &Prop_ordered_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#prop_ordered_arg}.
 * @param ctx the parse tree
 */
fn exit_prop_ordered_arg(&mut self, _ctx: &Prop_ordered_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#prop_named_arg}.
 * @param ctx the parse tree
 */
fn enter_prop_named_arg(&mut self, _ctx: &Prop_named_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#prop_named_arg}.
 * @param ctx the parse tree
 */
fn exit_prop_named_arg(&mut self, _ctx: &Prop_named_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_actual_arg}.
 * @param ctx the parse tree
 */
fn enter_property_actual_arg(&mut self, _ctx: &Property_actual_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_actual_arg}.
 * @param ctx the parse tree
 */
fn exit_property_actual_arg(&mut self, _ctx: &Property_actual_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assertion_item_declaration}.
 * @param ctx the parse tree
 */
fn enter_assertion_item_declaration(&mut self, _ctx: &Assertion_item_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assertion_item_declaration}.
 * @param ctx the parse tree
 */
fn exit_assertion_item_declaration(&mut self, _ctx: &Assertion_item_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_declaration}.
 * @param ctx the parse tree
 */
fn enter_property_declaration(&mut self, _ctx: &Property_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_declaration}.
 * @param ctx the parse tree
 */
fn exit_property_declaration(&mut self, _ctx: &Property_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_name}.
 * @param ctx the parse tree
 */
fn enter_property_name(&mut self, _ctx: &Property_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_name}.
 * @param ctx the parse tree
 */
fn exit_property_name(&mut self, _ctx: &Property_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#prop_port_list}.
 * @param ctx the parse tree
 */
fn enter_prop_port_list(&mut self, _ctx: &Prop_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#prop_port_list}.
 * @param ctx the parse tree
 */
fn exit_prop_port_list(&mut self, _ctx: &Prop_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_port_list}.
 * @param ctx the parse tree
 */
fn enter_property_port_list(&mut self, _ctx: &Property_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_port_list}.
 * @param ctx the parse tree
 */
fn exit_property_port_list(&mut self, _ctx: &Property_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_port_item}.
 * @param ctx the parse tree
 */
fn enter_property_port_item(&mut self, _ctx: &Property_port_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_port_item}.
 * @param ctx the parse tree
 */
fn exit_property_port_item(&mut self, _ctx: &Property_port_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#prop_port_item_local}.
 * @param ctx the parse tree
 */
fn enter_prop_port_item_local(&mut self, _ctx: &Prop_port_item_localContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#prop_port_item_local}.
 * @param ctx the parse tree
 */
fn exit_prop_port_item_local(&mut self, _ctx: &Prop_port_item_localContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_lvar_port_direction}.
 * @param ctx the parse tree
 */
fn enter_property_lvar_port_direction(&mut self, _ctx: &Property_lvar_port_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_lvar_port_direction}.
 * @param ctx the parse tree
 */
fn exit_property_lvar_port_direction(&mut self, _ctx: &Property_lvar_port_directionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_formal_type}.
 * @param ctx the parse tree
 */
fn enter_property_formal_type(&mut self, _ctx: &Property_formal_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_formal_type}.
 * @param ctx the parse tree
 */
fn exit_property_formal_type(&mut self, _ctx: &Property_formal_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_spec}.
 * @param ctx the parse tree
 */
fn enter_property_spec(&mut self, _ctx: &Property_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_spec}.
 * @param ctx the parse tree
 */
fn exit_property_spec(&mut self, _ctx: &Property_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_expr}.
 * @param ctx the parse tree
 */
fn enter_property_expr(&mut self, _ctx: &Property_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_expr}.
 * @param ctx the parse tree
 */
fn exit_property_expr(&mut self, _ctx: &Property_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_case_item}.
 * @param ctx the parse tree
 */
fn enter_property_case_item(&mut self, _ctx: &Property_case_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_case_item}.
 * @param ctx the parse tree
 */
fn exit_property_case_item(&mut self, _ctx: &Property_case_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_declaration}.
 * @param ctx the parse tree
 */
fn enter_sequence_declaration(&mut self, _ctx: &Sequence_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_declaration}.
 * @param ctx the parse tree
 */
fn exit_sequence_declaration(&mut self, _ctx: &Sequence_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_name}.
 * @param ctx the parse tree
 */
fn enter_sequence_name(&mut self, _ctx: &Sequence_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_name}.
 * @param ctx the parse tree
 */
fn exit_sequence_name(&mut self, _ctx: &Sequence_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_port_list}.
 * @param ctx the parse tree
 */
fn enter_seq_port_list(&mut self, _ctx: &Seq_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_port_list}.
 * @param ctx the parse tree
 */
fn exit_seq_port_list(&mut self, _ctx: &Seq_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_port_list}.
 * @param ctx the parse tree
 */
fn enter_sequence_port_list(&mut self, _ctx: &Sequence_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_port_list}.
 * @param ctx the parse tree
 */
fn exit_sequence_port_list(&mut self, _ctx: &Sequence_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_port_item}.
 * @param ctx the parse tree
 */
fn enter_sequence_port_item(&mut self, _ctx: &Sequence_port_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_port_item}.
 * @param ctx the parse tree
 */
fn exit_sequence_port_item(&mut self, _ctx: &Sequence_port_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_port_item_local}.
 * @param ctx the parse tree
 */
fn enter_seq_port_item_local(&mut self, _ctx: &Seq_port_item_localContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_port_item_local}.
 * @param ctx the parse tree
 */
fn exit_seq_port_item_local(&mut self, _ctx: &Seq_port_item_localContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_lvar_port_direction}.
 * @param ctx the parse tree
 */
fn enter_sequence_lvar_port_direction(&mut self, _ctx: &Sequence_lvar_port_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_lvar_port_direction}.
 * @param ctx the parse tree
 */
fn exit_sequence_lvar_port_direction(&mut self, _ctx: &Sequence_lvar_port_directionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_formal_type}.
 * @param ctx the parse tree
 */
fn enter_sequence_formal_type(&mut self, _ctx: &Sequence_formal_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_formal_type}.
 * @param ctx the parse tree
 */
fn exit_sequence_formal_type(&mut self, _ctx: &Sequence_formal_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_expr}.
 * @param ctx the parse tree
 */
fn enter_sequence_expr(&mut self, _ctx: &Sequence_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_expr}.
 * @param ctx the parse tree
 */
fn exit_sequence_expr(&mut self, _ctx: &Sequence_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cycle_delay_range}.
 * @param ctx the parse tree
 */
fn enter_cycle_delay_range(&mut self, _ctx: &Cycle_delay_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cycle_delay_range}.
 * @param ctx the parse tree
 */
fn exit_cycle_delay_range(&mut self, _ctx: &Cycle_delay_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_method_call}.
 * @param ctx the parse tree
 */
fn enter_sequence_method_call(&mut self, _ctx: &Sequence_method_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_method_call}.
 * @param ctx the parse tree
 */
fn exit_sequence_method_call(&mut self, _ctx: &Sequence_method_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_match_item}.
 * @param ctx the parse tree
 */
fn enter_sequence_match_item(&mut self, _ctx: &Sequence_match_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_match_item}.
 * @param ctx the parse tree
 */
fn exit_sequence_match_item(&mut self, _ctx: &Sequence_match_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_instance}.
 * @param ctx the parse tree
 */
fn enter_sequence_instance(&mut self, _ctx: &Sequence_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_instance}.
 * @param ctx the parse tree
 */
fn exit_sequence_instance(&mut self, _ctx: &Sequence_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_arg_list}.
 * @param ctx the parse tree
 */
fn enter_seq_arg_list(&mut self, _ctx: &Seq_arg_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_arg_list}.
 * @param ctx the parse tree
 */
fn exit_seq_arg_list(&mut self, _ctx: &Seq_arg_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_list_of_arguments}.
 * @param ctx the parse tree
 */
fn enter_sequence_list_of_arguments(&mut self, _ctx: &Sequence_list_of_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_list_of_arguments}.
 * @param ctx the parse tree
 */
fn exit_sequence_list_of_arguments(&mut self, _ctx: &Sequence_list_of_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_ordered_arg}.
 * @param ctx the parse tree
 */
fn enter_seq_ordered_arg(&mut self, _ctx: &Seq_ordered_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_ordered_arg}.
 * @param ctx the parse tree
 */
fn exit_seq_ordered_arg(&mut self, _ctx: &Seq_ordered_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_named_arg}.
 * @param ctx the parse tree
 */
fn enter_seq_named_arg(&mut self, _ctx: &Seq_named_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_named_arg}.
 * @param ctx the parse tree
 */
fn exit_seq_named_arg(&mut self, _ctx: &Seq_named_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_actual_arg}.
 * @param ctx the parse tree
 */
fn enter_sequence_actual_arg(&mut self, _ctx: &Sequence_actual_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_actual_arg}.
 * @param ctx the parse tree
 */
fn exit_sequence_actual_arg(&mut self, _ctx: &Sequence_actual_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#boolean_abbrev}.
 * @param ctx the parse tree
 */
fn enter_boolean_abbrev(&mut self, _ctx: &Boolean_abbrevContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#boolean_abbrev}.
 * @param ctx the parse tree
 */
fn exit_boolean_abbrev(&mut self, _ctx: &Boolean_abbrevContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_abbrev}.
 * @param ctx the parse tree
 */
fn enter_sequence_abbrev(&mut self, _ctx: &Sequence_abbrevContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_abbrev}.
 * @param ctx the parse tree
 */
fn exit_sequence_abbrev(&mut self, _ctx: &Sequence_abbrevContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#consecutive_repetition}.
 * @param ctx the parse tree
 */
fn enter_consecutive_repetition(&mut self, _ctx: &Consecutive_repetitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#consecutive_repetition}.
 * @param ctx the parse tree
 */
fn exit_consecutive_repetition(&mut self, _ctx: &Consecutive_repetitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#non_consecutive_repetition}.
 * @param ctx the parse tree
 */
fn enter_non_consecutive_repetition(&mut self, _ctx: &Non_consecutive_repetitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#non_consecutive_repetition}.
 * @param ctx the parse tree
 */
fn exit_non_consecutive_repetition(&mut self, _ctx: &Non_consecutive_repetitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#goto_repetition}.
 * @param ctx the parse tree
 */
fn enter_goto_repetition(&mut self, _ctx: &Goto_repetitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#goto_repetition}.
 * @param ctx the parse tree
 */
fn exit_goto_repetition(&mut self, _ctx: &Goto_repetitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#const_or_range_expression}.
 * @param ctx the parse tree
 */
fn enter_const_or_range_expression(&mut self, _ctx: &Const_or_range_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#const_or_range_expression}.
 * @param ctx the parse tree
 */
fn exit_const_or_range_expression(&mut self, _ctx: &Const_or_range_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cycle_delay_const_range_expression}.
 * @param ctx the parse tree
 */
fn enter_cycle_delay_const_range_expression(&mut self, _ctx: &Cycle_delay_const_range_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cycle_delay_const_range_expression}.
 * @param ctx the parse tree
 */
fn exit_cycle_delay_const_range_expression(&mut self, _ctx: &Cycle_delay_const_range_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#expression_or_dist}.
 * @param ctx the parse tree
 */
fn enter_expression_or_dist(&mut self, _ctx: &Expression_or_distContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#expression_or_dist}.
 * @param ctx the parse tree
 */
fn exit_expression_or_dist(&mut self, _ctx: &Expression_or_distContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assertion_variable_declaration}.
 * @param ctx the parse tree
 */
fn enter_assertion_variable_declaration(&mut self, _ctx: &Assertion_variable_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assertion_variable_declaration}.
 * @param ctx the parse tree
 */
fn exit_assertion_variable_declaration(&mut self, _ctx: &Assertion_variable_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#covergroup_declaration}.
 * @param ctx the parse tree
 */
fn enter_covergroup_declaration(&mut self, _ctx: &Covergroup_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#covergroup_declaration}.
 * @param ctx the parse tree
 */
fn exit_covergroup_declaration(&mut self, _ctx: &Covergroup_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#covergroup_name}.
 * @param ctx the parse tree
 */
fn enter_covergroup_name(&mut self, _ctx: &Covergroup_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#covergroup_name}.
 * @param ctx the parse tree
 */
fn exit_covergroup_name(&mut self, _ctx: &Covergroup_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#coverage_spec_or_option}.
 * @param ctx the parse tree
 */
fn enter_coverage_spec_or_option(&mut self, _ctx: &Coverage_spec_or_optionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#coverage_spec_or_option}.
 * @param ctx the parse tree
 */
fn exit_coverage_spec_or_option(&mut self, _ctx: &Coverage_spec_or_optionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#coverage_option}.
 * @param ctx the parse tree
 */
fn enter_coverage_option(&mut self, _ctx: &Coverage_optionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#coverage_option}.
 * @param ctx the parse tree
 */
fn exit_coverage_option(&mut self, _ctx: &Coverage_optionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#coverage_spec}.
 * @param ctx the parse tree
 */
fn enter_coverage_spec(&mut self, _ctx: &Coverage_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#coverage_spec}.
 * @param ctx the parse tree
 */
fn exit_coverage_spec(&mut self, _ctx: &Coverage_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#coverage_event}.
 * @param ctx the parse tree
 */
fn enter_coverage_event(&mut self, _ctx: &Coverage_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#coverage_event}.
 * @param ctx the parse tree
 */
fn exit_coverage_event(&mut self, _ctx: &Coverage_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#block_event_expression}.
 * @param ctx the parse tree
 */
fn enter_block_event_expression(&mut self, _ctx: &Block_event_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#block_event_expression}.
 * @param ctx the parse tree
 */
fn exit_block_event_expression(&mut self, _ctx: &Block_event_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hierarchical_btf_identifier}.
 * @param ctx the parse tree
 */
fn enter_hierarchical_btf_identifier(&mut self, _ctx: &Hierarchical_btf_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hierarchical_btf_identifier}.
 * @param ctx the parse tree
 */
fn exit_hierarchical_btf_identifier(&mut self, _ctx: &Hierarchical_btf_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cover_point}.
 * @param ctx the parse tree
 */
fn enter_cover_point(&mut self, _ctx: &Cover_pointContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cover_point}.
 * @param ctx the parse tree
 */
fn exit_cover_point(&mut self, _ctx: &Cover_pointContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cover_point_label}.
 * @param ctx the parse tree
 */
fn enter_cover_point_label(&mut self, _ctx: &Cover_point_labelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cover_point_label}.
 * @param ctx the parse tree
 */
fn exit_cover_point_label(&mut self, _ctx: &Cover_point_labelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bins_or_empty}.
 * @param ctx the parse tree
 */
fn enter_bins_or_empty(&mut self, _ctx: &Bins_or_emptyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bins_or_empty}.
 * @param ctx the parse tree
 */
fn exit_bins_or_empty(&mut self, _ctx: &Bins_or_emptyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bins_or_options}.
 * @param ctx the parse tree
 */
fn enter_bins_or_options(&mut self, _ctx: &Bins_or_optionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bins_or_options}.
 * @param ctx the parse tree
 */
fn exit_bins_or_options(&mut self, _ctx: &Bins_or_optionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bin_array_size}.
 * @param ctx the parse tree
 */
fn enter_bin_array_size(&mut self, _ctx: &Bin_array_sizeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bin_array_size}.
 * @param ctx the parse tree
 */
fn exit_bin_array_size(&mut self, _ctx: &Bin_array_sizeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bins_keyword}.
 * @param ctx the parse tree
 */
fn enter_bins_keyword(&mut self, _ctx: &Bins_keywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bins_keyword}.
 * @param ctx the parse tree
 */
fn exit_bins_keyword(&mut self, _ctx: &Bins_keywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#trans_list}.
 * @param ctx the parse tree
 */
fn enter_trans_list(&mut self, _ctx: &Trans_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#trans_list}.
 * @param ctx the parse tree
 */
fn exit_trans_list(&mut self, _ctx: &Trans_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#trans_set}.
 * @param ctx the parse tree
 */
fn enter_trans_set(&mut self, _ctx: &Trans_setContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#trans_set}.
 * @param ctx the parse tree
 */
fn exit_trans_set(&mut self, _ctx: &Trans_setContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#trans_range_list}.
 * @param ctx the parse tree
 */
fn enter_trans_range_list(&mut self, _ctx: &Trans_range_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#trans_range_list}.
 * @param ctx the parse tree
 */
fn exit_trans_range_list(&mut self, _ctx: &Trans_range_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#trans_item}.
 * @param ctx the parse tree
 */
fn enter_trans_item(&mut self, _ctx: &Trans_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#trans_item}.
 * @param ctx the parse tree
 */
fn exit_trans_item(&mut self, _ctx: &Trans_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#repeat_range}.
 * @param ctx the parse tree
 */
fn enter_repeat_range(&mut self, _ctx: &Repeat_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#repeat_range}.
 * @param ctx the parse tree
 */
fn exit_repeat_range(&mut self, _ctx: &Repeat_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cover_cross}.
 * @param ctx the parse tree
 */
fn enter_cover_cross(&mut self, _ctx: &Cover_crossContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cover_cross}.
 * @param ctx the parse tree
 */
fn exit_cover_cross(&mut self, _ctx: &Cover_crossContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cross_label}.
 * @param ctx the parse tree
 */
fn enter_cross_label(&mut self, _ctx: &Cross_labelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cross_label}.
 * @param ctx the parse tree
 */
fn exit_cross_label(&mut self, _ctx: &Cross_labelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_cross_items}.
 * @param ctx the parse tree
 */
fn enter_list_of_cross_items(&mut self, _ctx: &List_of_cross_itemsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_cross_items}.
 * @param ctx the parse tree
 */
fn exit_list_of_cross_items(&mut self, _ctx: &List_of_cross_itemsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cross_item}.
 * @param ctx the parse tree
 */
fn enter_cross_item(&mut self, _ctx: &Cross_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cross_item}.
 * @param ctx the parse tree
 */
fn exit_cross_item(&mut self, _ctx: &Cross_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cross_body}.
 * @param ctx the parse tree
 */
fn enter_cross_body(&mut self, _ctx: &Cross_bodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cross_body}.
 * @param ctx the parse tree
 */
fn exit_cross_body(&mut self, _ctx: &Cross_bodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cross_body_item}.
 * @param ctx the parse tree
 */
fn enter_cross_body_item(&mut self, _ctx: &Cross_body_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cross_body_item}.
 * @param ctx the parse tree
 */
fn exit_cross_body_item(&mut self, _ctx: &Cross_body_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bins_selection_or_option}.
 * @param ctx the parse tree
 */
fn enter_bins_selection_or_option(&mut self, _ctx: &Bins_selection_or_optionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bins_selection_or_option}.
 * @param ctx the parse tree
 */
fn exit_bins_selection_or_option(&mut self, _ctx: &Bins_selection_or_optionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bins_selection}.
 * @param ctx the parse tree
 */
fn enter_bins_selection(&mut self, _ctx: &Bins_selectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bins_selection}.
 * @param ctx the parse tree
 */
fn exit_bins_selection(&mut self, _ctx: &Bins_selectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#select_expression}.
 * @param ctx the parse tree
 */
fn enter_select_expression(&mut self, _ctx: &Select_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#select_expression}.
 * @param ctx the parse tree
 */
fn exit_select_expression(&mut self, _ctx: &Select_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#select_condition}.
 * @param ctx the parse tree
 */
fn enter_select_condition(&mut self, _ctx: &Select_conditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#select_condition}.
 * @param ctx the parse tree
 */
fn exit_select_condition(&mut self, _ctx: &Select_conditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bins_expression}.
 * @param ctx the parse tree
 */
fn enter_bins_expression(&mut self, _ctx: &Bins_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bins_expression}.
 * @param ctx the parse tree
 */
fn exit_bins_expression(&mut self, _ctx: &Bins_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#covergroup_range_list}.
 * @param ctx the parse tree
 */
fn enter_covergroup_range_list(&mut self, _ctx: &Covergroup_range_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#covergroup_range_list}.
 * @param ctx the parse tree
 */
fn exit_covergroup_range_list(&mut self, _ctx: &Covergroup_range_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#covergroup_value_range}.
 * @param ctx the parse tree
 */
fn enter_covergroup_value_range(&mut self, _ctx: &Covergroup_value_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#covergroup_value_range}.
 * @param ctx the parse tree
 */
fn exit_covergroup_value_range(&mut self, _ctx: &Covergroup_value_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#with_covergroup_expression}.
 * @param ctx the parse tree
 */
fn enter_with_covergroup_expression(&mut self, _ctx: &With_covergroup_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#with_covergroup_expression}.
 * @param ctx the parse tree
 */
fn exit_with_covergroup_expression(&mut self, _ctx: &With_covergroup_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#set_covergroup_expression}.
 * @param ctx the parse tree
 */
fn enter_set_covergroup_expression(&mut self, _ctx: &Set_covergroup_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#set_covergroup_expression}.
 * @param ctx the parse tree
 */
fn exit_set_covergroup_expression(&mut self, _ctx: &Set_covergroup_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#integer_covergroup_expression}.
 * @param ctx the parse tree
 */
fn enter_integer_covergroup_expression(&mut self, _ctx: &Integer_covergroup_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#integer_covergroup_expression}.
 * @param ctx the parse tree
 */
fn exit_integer_covergroup_expression(&mut self, _ctx: &Integer_covergroup_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cross_set_expression}.
 * @param ctx the parse tree
 */
fn enter_cross_set_expression(&mut self, _ctx: &Cross_set_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cross_set_expression}.
 * @param ctx the parse tree
 */
fn exit_cross_set_expression(&mut self, _ctx: &Cross_set_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#covergroup_expression}.
 * @param ctx the parse tree
 */
fn enter_covergroup_expression(&mut self, _ctx: &Covergroup_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#covergroup_expression}.
 * @param ctx the parse tree
 */
fn exit_covergroup_expression(&mut self, _ctx: &Covergroup_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#let_declaration}.
 * @param ctx the parse tree
 */
fn enter_let_declaration(&mut self, _ctx: &Let_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#let_declaration}.
 * @param ctx the parse tree
 */
fn exit_let_declaration(&mut self, _ctx: &Let_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#let_ports}.
 * @param ctx the parse tree
 */
fn enter_let_ports(&mut self, _ctx: &Let_portsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#let_ports}.
 * @param ctx the parse tree
 */
fn exit_let_ports(&mut self, _ctx: &Let_portsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#let_identifier}.
 * @param ctx the parse tree
 */
fn enter_let_identifier(&mut self, _ctx: &Let_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#let_identifier}.
 * @param ctx the parse tree
 */
fn exit_let_identifier(&mut self, _ctx: &Let_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#let_port_list}.
 * @param ctx the parse tree
 */
fn enter_let_port_list(&mut self, _ctx: &Let_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#let_port_list}.
 * @param ctx the parse tree
 */
fn exit_let_port_list(&mut self, _ctx: &Let_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#let_port_item}.
 * @param ctx the parse tree
 */
fn enter_let_port_item(&mut self, _ctx: &Let_port_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#let_port_item}.
 * @param ctx the parse tree
 */
fn exit_let_port_item(&mut self, _ctx: &Let_port_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#let_formal_type}.
 * @param ctx the parse tree
 */
fn enter_let_formal_type(&mut self, _ctx: &Let_formal_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#let_formal_type}.
 * @param ctx the parse tree
 */
fn exit_let_formal_type(&mut self, _ctx: &Let_formal_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#gate_instantiation}.
 * @param ctx the parse tree
 */
fn enter_gate_instantiation(&mut self, _ctx: &Gate_instantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#gate_instantiation}.
 * @param ctx the parse tree
 */
fn exit_gate_instantiation(&mut self, _ctx: &Gate_instantiationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cmos_switch_instance}.
 * @param ctx the parse tree
 */
fn enter_cmos_switch_instance(&mut self, _ctx: &Cmos_switch_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cmos_switch_instance}.
 * @param ctx the parse tree
 */
fn exit_cmos_switch_instance(&mut self, _ctx: &Cmos_switch_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enable_gate_instance}.
 * @param ctx the parse tree
 */
fn enter_enable_gate_instance(&mut self, _ctx: &Enable_gate_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enable_gate_instance}.
 * @param ctx the parse tree
 */
fn exit_enable_gate_instance(&mut self, _ctx: &Enable_gate_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#mos_switch_instance}.
 * @param ctx the parse tree
 */
fn enter_mos_switch_instance(&mut self, _ctx: &Mos_switch_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#mos_switch_instance}.
 * @param ctx the parse tree
 */
fn exit_mos_switch_instance(&mut self, _ctx: &Mos_switch_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#n_input_gate_instance}.
 * @param ctx the parse tree
 */
fn enter_n_input_gate_instance(&mut self, _ctx: &N_input_gate_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#n_input_gate_instance}.
 * @param ctx the parse tree
 */
fn exit_n_input_gate_instance(&mut self, _ctx: &N_input_gate_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#n_output_gate_instance}.
 * @param ctx the parse tree
 */
fn enter_n_output_gate_instance(&mut self, _ctx: &N_output_gate_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#n_output_gate_instance}.
 * @param ctx the parse tree
 */
fn exit_n_output_gate_instance(&mut self, _ctx: &N_output_gate_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pass_switch_instance}.
 * @param ctx the parse tree
 */
fn enter_pass_switch_instance(&mut self, _ctx: &Pass_switch_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pass_switch_instance}.
 * @param ctx the parse tree
 */
fn exit_pass_switch_instance(&mut self, _ctx: &Pass_switch_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pass_enable_switch_instance}.
 * @param ctx the parse tree
 */
fn enter_pass_enable_switch_instance(&mut self, _ctx: &Pass_enable_switch_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pass_enable_switch_instance}.
 * @param ctx the parse tree
 */
fn exit_pass_enable_switch_instance(&mut self, _ctx: &Pass_enable_switch_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pull_gate_instance}.
 * @param ctx the parse tree
 */
fn enter_pull_gate_instance(&mut self, _ctx: &Pull_gate_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pull_gate_instance}.
 * @param ctx the parse tree
 */
fn exit_pull_gate_instance(&mut self, _ctx: &Pull_gate_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pulldown_strength}.
 * @param ctx the parse tree
 */
fn enter_pulldown_strength(&mut self, _ctx: &Pulldown_strengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pulldown_strength}.
 * @param ctx the parse tree
 */
fn exit_pulldown_strength(&mut self, _ctx: &Pulldown_strengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pullup_strength}.
 * @param ctx the parse tree
 */
fn enter_pullup_strength(&mut self, _ctx: &Pullup_strengthContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pullup_strength}.
 * @param ctx the parse tree
 */
fn exit_pullup_strength(&mut self, _ctx: &Pullup_strengthContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enable_terminal}.
 * @param ctx the parse tree
 */
fn enter_enable_terminal(&mut self, _ctx: &Enable_terminalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enable_terminal}.
 * @param ctx the parse tree
 */
fn exit_enable_terminal(&mut self, _ctx: &Enable_terminalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#inout_terminal}.
 * @param ctx the parse tree
 */
fn enter_inout_terminal(&mut self, _ctx: &Inout_terminalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#inout_terminal}.
 * @param ctx the parse tree
 */
fn exit_inout_terminal(&mut self, _ctx: &Inout_terminalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#input_terminal}.
 * @param ctx the parse tree
 */
fn enter_input_terminal(&mut self, _ctx: &Input_terminalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#input_terminal}.
 * @param ctx the parse tree
 */
fn exit_input_terminal(&mut self, _ctx: &Input_terminalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ncontrol_terminal}.
 * @param ctx the parse tree
 */
fn enter_ncontrol_terminal(&mut self, _ctx: &Ncontrol_terminalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ncontrol_terminal}.
 * @param ctx the parse tree
 */
fn exit_ncontrol_terminal(&mut self, _ctx: &Ncontrol_terminalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#output_terminal}.
 * @param ctx the parse tree
 */
fn enter_output_terminal(&mut self, _ctx: &Output_terminalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#output_terminal}.
 * @param ctx the parse tree
 */
fn exit_output_terminal(&mut self, _ctx: &Output_terminalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pcontrol_terminal}.
 * @param ctx the parse tree
 */
fn enter_pcontrol_terminal(&mut self, _ctx: &Pcontrol_terminalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pcontrol_terminal}.
 * @param ctx the parse tree
 */
fn exit_pcontrol_terminal(&mut self, _ctx: &Pcontrol_terminalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cmos_switchtype}.
 * @param ctx the parse tree
 */
fn enter_cmos_switchtype(&mut self, _ctx: &Cmos_switchtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cmos_switchtype}.
 * @param ctx the parse tree
 */
fn exit_cmos_switchtype(&mut self, _ctx: &Cmos_switchtypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enable_gatetype}.
 * @param ctx the parse tree
 */
fn enter_enable_gatetype(&mut self, _ctx: &Enable_gatetypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enable_gatetype}.
 * @param ctx the parse tree
 */
fn exit_enable_gatetype(&mut self, _ctx: &Enable_gatetypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#mos_switchtype}.
 * @param ctx the parse tree
 */
fn enter_mos_switchtype(&mut self, _ctx: &Mos_switchtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#mos_switchtype}.
 * @param ctx the parse tree
 */
fn exit_mos_switchtype(&mut self, _ctx: &Mos_switchtypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#n_input_gatetype}.
 * @param ctx the parse tree
 */
fn enter_n_input_gatetype(&mut self, _ctx: &N_input_gatetypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#n_input_gatetype}.
 * @param ctx the parse tree
 */
fn exit_n_input_gatetype(&mut self, _ctx: &N_input_gatetypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#n_output_gatetype}.
 * @param ctx the parse tree
 */
fn enter_n_output_gatetype(&mut self, _ctx: &N_output_gatetypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#n_output_gatetype}.
 * @param ctx the parse tree
 */
fn exit_n_output_gatetype(&mut self, _ctx: &N_output_gatetypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pass_en_switchtype}.
 * @param ctx the parse tree
 */
fn enter_pass_en_switchtype(&mut self, _ctx: &Pass_en_switchtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pass_en_switchtype}.
 * @param ctx the parse tree
 */
fn exit_pass_en_switchtype(&mut self, _ctx: &Pass_en_switchtypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pass_switchtype}.
 * @param ctx the parse tree
 */
fn enter_pass_switchtype(&mut self, _ctx: &Pass_switchtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pass_switchtype}.
 * @param ctx the parse tree
 */
fn exit_pass_switchtype(&mut self, _ctx: &Pass_switchtypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_program_interface_instantiation}.
 * @param ctx the parse tree
 */
fn enter_module_program_interface_instantiation(&mut self, _ctx: &Module_program_interface_instantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_program_interface_instantiation}.
 * @param ctx the parse tree
 */
fn exit_module_program_interface_instantiation(&mut self, _ctx: &Module_program_interface_instantiationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parameter_value_assignment}.
 * @param ctx the parse tree
 */
fn enter_parameter_value_assignment(&mut self, _ctx: &Parameter_value_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parameter_value_assignment}.
 * @param ctx the parse tree
 */
fn exit_parameter_value_assignment(&mut self, _ctx: &Parameter_value_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_parameter_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_parameter_assignments(&mut self, _ctx: &List_of_parameter_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_parameter_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_parameter_assignments(&mut self, _ctx: &List_of_parameter_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ordered_parameter_assignment}.
 * @param ctx the parse tree
 */
fn enter_ordered_parameter_assignment(&mut self, _ctx: &Ordered_parameter_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ordered_parameter_assignment}.
 * @param ctx the parse tree
 */
fn exit_ordered_parameter_assignment(&mut self, _ctx: &Ordered_parameter_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#named_parameter_assignment}.
 * @param ctx the parse tree
 */
fn enter_named_parameter_assignment(&mut self, _ctx: &Named_parameter_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#named_parameter_assignment}.
 * @param ctx the parse tree
 */
fn exit_named_parameter_assignment(&mut self, _ctx: &Named_parameter_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hierarchical_instance}.
 * @param ctx the parse tree
 */
fn enter_hierarchical_instance(&mut self, _ctx: &Hierarchical_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hierarchical_instance}.
 * @param ctx the parse tree
 */
fn exit_hierarchical_instance(&mut self, _ctx: &Hierarchical_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#name_of_instance}.
 * @param ctx the parse tree
 */
fn enter_name_of_instance(&mut self, _ctx: &Name_of_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#name_of_instance}.
 * @param ctx the parse tree
 */
fn exit_name_of_instance(&mut self, _ctx: &Name_of_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_port_connections}.
 * @param ctx the parse tree
 */
fn enter_list_of_port_connections(&mut self, _ctx: &List_of_port_connectionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_port_connections}.
 * @param ctx the parse tree
 */
fn exit_list_of_port_connections(&mut self, _ctx: &List_of_port_connectionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ordered_port_connection}.
 * @param ctx the parse tree
 */
fn enter_ordered_port_connection(&mut self, _ctx: &Ordered_port_connectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ordered_port_connection}.
 * @param ctx the parse tree
 */
fn exit_ordered_port_connection(&mut self, _ctx: &Ordered_port_connectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#named_port_connection}.
 * @param ctx the parse tree
 */
fn enter_named_port_connection(&mut self, _ctx: &Named_port_connectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#named_port_connection}.
 * @param ctx the parse tree
 */
fn exit_named_port_connection(&mut self, _ctx: &Named_port_connectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_assign}.
 * @param ctx the parse tree
 */
fn enter_port_assign(&mut self, _ctx: &Port_assignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_assign}.
 * @param ctx the parse tree
 */
fn exit_port_assign(&mut self, _ctx: &Port_assignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_instantiation}.
 * @param ctx the parse tree
 */
fn enter_checker_instantiation(&mut self, _ctx: &Checker_instantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_instantiation}.
 * @param ctx the parse tree
 */
fn exit_checker_instantiation(&mut self, _ctx: &Checker_instantiationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_checker_port_connections}.
 * @param ctx the parse tree
 */
fn enter_list_of_checker_port_connections(&mut self, _ctx: &List_of_checker_port_connectionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_checker_port_connections}.
 * @param ctx the parse tree
 */
fn exit_list_of_checker_port_connections(&mut self, _ctx: &List_of_checker_port_connectionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ordered_checker_port_connection}.
 * @param ctx the parse tree
 */
fn enter_ordered_checker_port_connection(&mut self, _ctx: &Ordered_checker_port_connectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ordered_checker_port_connection}.
 * @param ctx the parse tree
 */
fn exit_ordered_checker_port_connection(&mut self, _ctx: &Ordered_checker_port_connectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#named_checker_port_connection}.
 * @param ctx the parse tree
 */
fn enter_named_checker_port_connection(&mut self, _ctx: &Named_checker_port_connectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#named_checker_port_connection}.
 * @param ctx the parse tree
 */
fn exit_named_checker_port_connection(&mut self, _ctx: &Named_checker_port_connectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_port_assign}.
 * @param ctx the parse tree
 */
fn enter_checker_port_assign(&mut self, _ctx: &Checker_port_assignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_port_assign}.
 * @param ctx the parse tree
 */
fn exit_checker_port_assign(&mut self, _ctx: &Checker_port_assignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#generate_region}.
 * @param ctx the parse tree
 */
fn enter_generate_region(&mut self, _ctx: &Generate_regionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#generate_region}.
 * @param ctx the parse tree
 */
fn exit_generate_region(&mut self, _ctx: &Generate_regionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#loop_generate_construct}.
 * @param ctx the parse tree
 */
fn enter_loop_generate_construct(&mut self, _ctx: &Loop_generate_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#loop_generate_construct}.
 * @param ctx the parse tree
 */
fn exit_loop_generate_construct(&mut self, _ctx: &Loop_generate_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#genvar_initialization}.
 * @param ctx the parse tree
 */
fn enter_genvar_initialization(&mut self, _ctx: &Genvar_initializationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#genvar_initialization}.
 * @param ctx the parse tree
 */
fn exit_genvar_initialization(&mut self, _ctx: &Genvar_initializationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#genvar_iteration}.
 * @param ctx the parse tree
 */
fn enter_genvar_iteration(&mut self, _ctx: &Genvar_iterationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#genvar_iteration}.
 * @param ctx the parse tree
 */
fn exit_genvar_iteration(&mut self, _ctx: &Genvar_iterationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#conditional_generate_construct}.
 * @param ctx the parse tree
 */
fn enter_conditional_generate_construct(&mut self, _ctx: &Conditional_generate_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#conditional_generate_construct}.
 * @param ctx the parse tree
 */
fn exit_conditional_generate_construct(&mut self, _ctx: &Conditional_generate_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#if_generate_construct}.
 * @param ctx the parse tree
 */
fn enter_if_generate_construct(&mut self, _ctx: &If_generate_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#if_generate_construct}.
 * @param ctx the parse tree
 */
fn exit_if_generate_construct(&mut self, _ctx: &If_generate_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_generate_construct}.
 * @param ctx the parse tree
 */
fn enter_case_generate_construct(&mut self, _ctx: &Case_generate_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_generate_construct}.
 * @param ctx the parse tree
 */
fn exit_case_generate_construct(&mut self, _ctx: &Case_generate_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_generate_item}.
 * @param ctx the parse tree
 */
fn enter_case_generate_item(&mut self, _ctx: &Case_generate_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_generate_item}.
 * @param ctx the parse tree
 */
fn exit_case_generate_item(&mut self, _ctx: &Case_generate_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#generate_block}.
 * @param ctx the parse tree
 */
fn enter_generate_block(&mut self, _ctx: &Generate_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#generate_block}.
 * @param ctx the parse tree
 */
fn exit_generate_block(&mut self, _ctx: &Generate_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#generate_block_label}.
 * @param ctx the parse tree
 */
fn enter_generate_block_label(&mut self, _ctx: &Generate_block_labelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#generate_block_label}.
 * @param ctx the parse tree
 */
fn exit_generate_block_label(&mut self, _ctx: &Generate_block_labelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#generate_block_name}.
 * @param ctx the parse tree
 */
fn enter_generate_block_name(&mut self, _ctx: &Generate_block_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#generate_block_name}.
 * @param ctx the parse tree
 */
fn exit_generate_block_name(&mut self, _ctx: &Generate_block_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#generate_item}.
 * @param ctx the parse tree
 */
fn enter_generate_item(&mut self, _ctx: &Generate_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#generate_item}.
 * @param ctx the parse tree
 */
fn exit_generate_item(&mut self, _ctx: &Generate_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_nonansi_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_nonansi_declaration(&mut self, _ctx: &Udp_nonansi_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_nonansi_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_nonansi_declaration(&mut self, _ctx: &Udp_nonansi_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_ansi_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_ansi_declaration(&mut self, _ctx: &Udp_ansi_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_ansi_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_ansi_declaration(&mut self, _ctx: &Udp_ansi_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_declaration(&mut self, _ctx: &Udp_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_declaration(&mut self, _ctx: &Udp_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_name}.
 * @param ctx the parse tree
 */
fn enter_udp_name(&mut self, _ctx: &Udp_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_name}.
 * @param ctx the parse tree
 */
fn exit_udp_name(&mut self, _ctx: &Udp_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_port_list}.
 * @param ctx the parse tree
 */
fn enter_udp_port_list(&mut self, _ctx: &Udp_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_port_list}.
 * @param ctx the parse tree
 */
fn exit_udp_port_list(&mut self, _ctx: &Udp_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_declaration_port_list}.
 * @param ctx the parse tree
 */
fn enter_udp_declaration_port_list(&mut self, _ctx: &Udp_declaration_port_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_declaration_port_list}.
 * @param ctx the parse tree
 */
fn exit_udp_declaration_port_list(&mut self, _ctx: &Udp_declaration_port_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_port_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_port_declaration(&mut self, _ctx: &Udp_port_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_port_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_port_declaration(&mut self, _ctx: &Udp_port_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_output_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_output_declaration(&mut self, _ctx: &Udp_output_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_output_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_output_declaration(&mut self, _ctx: &Udp_output_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_input_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_input_declaration(&mut self, _ctx: &Udp_input_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_input_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_input_declaration(&mut self, _ctx: &Udp_input_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_reg_declaration}.
 * @param ctx the parse tree
 */
fn enter_udp_reg_declaration(&mut self, _ctx: &Udp_reg_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_reg_declaration}.
 * @param ctx the parse tree
 */
fn exit_udp_reg_declaration(&mut self, _ctx: &Udp_reg_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_body}.
 * @param ctx the parse tree
 */
fn enter_udp_body(&mut self, _ctx: &Udp_bodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_body}.
 * @param ctx the parse tree
 */
fn exit_udp_body(&mut self, _ctx: &Udp_bodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#combinational_body}.
 * @param ctx the parse tree
 */
fn enter_combinational_body(&mut self, _ctx: &Combinational_bodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#combinational_body}.
 * @param ctx the parse tree
 */
fn exit_combinational_body(&mut self, _ctx: &Combinational_bodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#combinational_entry}.
 * @param ctx the parse tree
 */
fn enter_combinational_entry(&mut self, _ctx: &Combinational_entryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#combinational_entry}.
 * @param ctx the parse tree
 */
fn exit_combinational_entry(&mut self, _ctx: &Combinational_entryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequential_body}.
 * @param ctx the parse tree
 */
fn enter_sequential_body(&mut self, _ctx: &Sequential_bodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequential_body}.
 * @param ctx the parse tree
 */
fn exit_sequential_body(&mut self, _ctx: &Sequential_bodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_initial_statement}.
 * @param ctx the parse tree
 */
fn enter_udp_initial_statement(&mut self, _ctx: &Udp_initial_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_initial_statement}.
 * @param ctx the parse tree
 */
fn exit_udp_initial_statement(&mut self, _ctx: &Udp_initial_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#init_val}.
 * @param ctx the parse tree
 */
fn enter_init_val(&mut self, _ctx: &Init_valContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#init_val}.
 * @param ctx the parse tree
 */
fn exit_init_val(&mut self, _ctx: &Init_valContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequential_entry}.
 * @param ctx the parse tree
 */
fn enter_sequential_entry(&mut self, _ctx: &Sequential_entryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequential_entry}.
 * @param ctx the parse tree
 */
fn exit_sequential_entry(&mut self, _ctx: &Sequential_entryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_input_list}.
 * @param ctx the parse tree
 */
fn enter_seq_input_list(&mut self, _ctx: &Seq_input_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_input_list}.
 * @param ctx the parse tree
 */
fn exit_seq_input_list(&mut self, _ctx: &Seq_input_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#level_input_list}.
 * @param ctx the parse tree
 */
fn enter_level_input_list(&mut self, _ctx: &Level_input_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#level_input_list}.
 * @param ctx the parse tree
 */
fn exit_level_input_list(&mut self, _ctx: &Level_input_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_input_list}.
 * @param ctx the parse tree
 */
fn enter_edge_input_list(&mut self, _ctx: &Edge_input_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_input_list}.
 * @param ctx the parse tree
 */
fn exit_edge_input_list(&mut self, _ctx: &Edge_input_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_indicator}.
 * @param ctx the parse tree
 */
fn enter_edge_indicator(&mut self, _ctx: &Edge_indicatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_indicator}.
 * @param ctx the parse tree
 */
fn exit_edge_indicator(&mut self, _ctx: &Edge_indicatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#current_state}.
 * @param ctx the parse tree
 */
fn enter_current_state(&mut self, _ctx: &Current_stateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#current_state}.
 * @param ctx the parse tree
 */
fn exit_current_state(&mut self, _ctx: &Current_stateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#next_state}.
 * @param ctx the parse tree
 */
fn enter_next_state(&mut self, _ctx: &Next_stateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#next_state}.
 * @param ctx the parse tree
 */
fn exit_next_state(&mut self, _ctx: &Next_stateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#output_symbol}.
 * @param ctx the parse tree
 */
fn enter_output_symbol(&mut self, _ctx: &Output_symbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#output_symbol}.
 * @param ctx the parse tree
 */
fn exit_output_symbol(&mut self, _ctx: &Output_symbolContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#level_symbol}.
 * @param ctx the parse tree
 */
fn enter_level_symbol(&mut self, _ctx: &Level_symbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#level_symbol}.
 * @param ctx the parse tree
 */
fn exit_level_symbol(&mut self, _ctx: &Level_symbolContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_symbol}.
 * @param ctx the parse tree
 */
fn enter_edge_symbol(&mut self, _ctx: &Edge_symbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_symbol}.
 * @param ctx the parse tree
 */
fn exit_edge_symbol(&mut self, _ctx: &Edge_symbolContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_instantiation}.
 * @param ctx the parse tree
 */
fn enter_udp_instantiation(&mut self, _ctx: &Udp_instantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_instantiation}.
 * @param ctx the parse tree
 */
fn exit_udp_instantiation(&mut self, _ctx: &Udp_instantiationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_instance}.
 * @param ctx the parse tree
 */
fn enter_udp_instance(&mut self, _ctx: &Udp_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_instance}.
 * @param ctx the parse tree
 */
fn exit_udp_instance(&mut self, _ctx: &Udp_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#continuous_assign}.
 * @param ctx the parse tree
 */
fn enter_continuous_assign(&mut self, _ctx: &Continuous_assignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#continuous_assign}.
 * @param ctx the parse tree
 */
fn exit_continuous_assign(&mut self, _ctx: &Continuous_assignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_net_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_net_assignments(&mut self, _ctx: &List_of_net_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_net_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_net_assignments(&mut self, _ctx: &List_of_net_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_variable_assignments}.
 * @param ctx the parse tree
 */
fn enter_list_of_variable_assignments(&mut self, _ctx: &List_of_variable_assignmentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_variable_assignments}.
 * @param ctx the parse tree
 */
fn exit_list_of_variable_assignments(&mut self, _ctx: &List_of_variable_assignmentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_alias}.
 * @param ctx the parse tree
 */
fn enter_net_alias(&mut self, _ctx: &Net_aliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_alias}.
 * @param ctx the parse tree
 */
fn exit_net_alias(&mut self, _ctx: &Net_aliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_assignment}.
 * @param ctx the parse tree
 */
fn enter_net_assignment(&mut self, _ctx: &Net_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_assignment}.
 * @param ctx the parse tree
 */
fn exit_net_assignment(&mut self, _ctx: &Net_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#initial_construct}.
 * @param ctx the parse tree
 */
fn enter_initial_construct(&mut self, _ctx: &Initial_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#initial_construct}.
 * @param ctx the parse tree
 */
fn exit_initial_construct(&mut self, _ctx: &Initial_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#always_construct}.
 * @param ctx the parse tree
 */
fn enter_always_construct(&mut self, _ctx: &Always_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#always_construct}.
 * @param ctx the parse tree
 */
fn exit_always_construct(&mut self, _ctx: &Always_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#always_keyword}.
 * @param ctx the parse tree
 */
fn enter_always_keyword(&mut self, _ctx: &Always_keywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#always_keyword}.
 * @param ctx the parse tree
 */
fn exit_always_keyword(&mut self, _ctx: &Always_keywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#final_construct}.
 * @param ctx the parse tree
 */
fn enter_final_construct(&mut self, _ctx: &Final_constructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#final_construct}.
 * @param ctx the parse tree
 */
fn exit_final_construct(&mut self, _ctx: &Final_constructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#blocking_assignment}.
 * @param ctx the parse tree
 */
fn enter_blocking_assignment(&mut self, _ctx: &Blocking_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#blocking_assignment}.
 * @param ctx the parse tree
 */
fn exit_blocking_assignment(&mut self, _ctx: &Blocking_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#operator_assignment}.
 * @param ctx the parse tree
 */
fn enter_operator_assignment(&mut self, _ctx: &Operator_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#operator_assignment}.
 * @param ctx the parse tree
 */
fn exit_operator_assignment(&mut self, _ctx: &Operator_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_operator}.
 * @param ctx the parse tree
 */
fn enter_assignment_operator(&mut self, _ctx: &Assignment_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_operator}.
 * @param ctx the parse tree
 */
fn exit_assignment_operator(&mut self, _ctx: &Assignment_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#nonblocking_assignment}.
 * @param ctx the parse tree
 */
fn enter_nonblocking_assignment(&mut self, _ctx: &Nonblocking_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#nonblocking_assignment}.
 * @param ctx the parse tree
 */
fn exit_nonblocking_assignment(&mut self, _ctx: &Nonblocking_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#procedural_continuous_assignment}.
 * @param ctx the parse tree
 */
fn enter_procedural_continuous_assignment(&mut self, _ctx: &Procedural_continuous_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#procedural_continuous_assignment}.
 * @param ctx the parse tree
 */
fn exit_procedural_continuous_assignment(&mut self, _ctx: &Procedural_continuous_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_assignment}.
 * @param ctx the parse tree
 */
fn enter_variable_assignment(&mut self, _ctx: &Variable_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_assignment}.
 * @param ctx the parse tree
 */
fn exit_variable_assignment(&mut self, _ctx: &Variable_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#action_block}.
 * @param ctx the parse tree
 */
fn enter_action_block(&mut self, _ctx: &Action_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#action_block}.
 * @param ctx the parse tree
 */
fn exit_action_block(&mut self, _ctx: &Action_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#seq_block}.
 * @param ctx the parse tree
 */
fn enter_seq_block(&mut self, _ctx: &Seq_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#seq_block}.
 * @param ctx the parse tree
 */
fn exit_seq_block(&mut self, _ctx: &Seq_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#block_name}.
 * @param ctx the parse tree
 */
fn enter_block_name(&mut self, _ctx: &Block_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#block_name}.
 * @param ctx the parse tree
 */
fn exit_block_name(&mut self, _ctx: &Block_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#par_block}.
 * @param ctx the parse tree
 */
fn enter_par_block(&mut self, _ctx: &Par_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#par_block}.
 * @param ctx the parse tree
 */
fn exit_par_block(&mut self, _ctx: &Par_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#join_keyword}.
 * @param ctx the parse tree
 */
fn enter_join_keyword(&mut self, _ctx: &Join_keywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#join_keyword}.
 * @param ctx the parse tree
 */
fn exit_join_keyword(&mut self, _ctx: &Join_keywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#statement_or_null}.
 * @param ctx the parse tree
 */
fn enter_statement_or_null(&mut self, _ctx: &Statement_or_nullContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#statement_or_null}.
 * @param ctx the parse tree
 */
fn exit_statement_or_null(&mut self, _ctx: &Statement_or_nullContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#statement_item}.
 * @param ctx the parse tree
 */
fn enter_statement_item(&mut self, _ctx: &Statement_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#statement_item}.
 * @param ctx the parse tree
 */
fn exit_statement_item(&mut self, _ctx: &Statement_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_statement}.
 * @param ctx the parse tree
 */
fn enter_function_statement(&mut self, _ctx: &Function_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_statement}.
 * @param ctx the parse tree
 */
fn exit_function_statement(&mut self, _ctx: &Function_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_statement_or_null}.
 * @param ctx the parse tree
 */
fn enter_function_statement_or_null(&mut self, _ctx: &Function_statement_or_nullContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_statement_or_null}.
 * @param ctx the parse tree
 */
fn exit_function_statement_or_null(&mut self, _ctx: &Function_statement_or_nullContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_identifier_list}.
 * @param ctx the parse tree
 */
fn enter_variable_identifier_list(&mut self, _ctx: &Variable_identifier_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_identifier_list}.
 * @param ctx the parse tree
 */
fn exit_variable_identifier_list(&mut self, _ctx: &Variable_identifier_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#procedural_timing_control_statement}.
 * @param ctx the parse tree
 */
fn enter_procedural_timing_control_statement(&mut self, _ctx: &Procedural_timing_control_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#procedural_timing_control_statement}.
 * @param ctx the parse tree
 */
fn exit_procedural_timing_control_statement(&mut self, _ctx: &Procedural_timing_control_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delay_or_event_control}.
 * @param ctx the parse tree
 */
fn enter_delay_or_event_control(&mut self, _ctx: &Delay_or_event_controlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delay_or_event_control}.
 * @param ctx the parse tree
 */
fn exit_delay_or_event_control(&mut self, _ctx: &Delay_or_event_controlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delay_control}.
 * @param ctx the parse tree
 */
fn enter_delay_control(&mut self, _ctx: &Delay_controlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delay_control}.
 * @param ctx the parse tree
 */
fn exit_delay_control(&mut self, _ctx: &Delay_controlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#event_control}.
 * @param ctx the parse tree
 */
fn enter_event_control(&mut self, _ctx: &Event_controlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#event_control}.
 * @param ctx the parse tree
 */
fn exit_event_control(&mut self, _ctx: &Event_controlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#event_expression}.
 * @param ctx the parse tree
 */
fn enter_event_expression(&mut self, _ctx: &Event_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#event_expression}.
 * @param ctx the parse tree
 */
fn exit_event_expression(&mut self, _ctx: &Event_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#procedural_timing_control}.
 * @param ctx the parse tree
 */
fn enter_procedural_timing_control(&mut self, _ctx: &Procedural_timing_controlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#procedural_timing_control}.
 * @param ctx the parse tree
 */
fn exit_procedural_timing_control(&mut self, _ctx: &Procedural_timing_controlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#jump_statement}.
 * @param ctx the parse tree
 */
fn enter_jump_statement(&mut self, _ctx: &Jump_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#jump_statement}.
 * @param ctx the parse tree
 */
fn exit_jump_statement(&mut self, _ctx: &Jump_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#wait_statement}.
 * @param ctx the parse tree
 */
fn enter_wait_statement(&mut self, _ctx: &Wait_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#wait_statement}.
 * @param ctx the parse tree
 */
fn exit_wait_statement(&mut self, _ctx: &Wait_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#event_trigger}.
 * @param ctx the parse tree
 */
fn enter_event_trigger(&mut self, _ctx: &Event_triggerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#event_trigger}.
 * @param ctx the parse tree
 */
fn exit_event_trigger(&mut self, _ctx: &Event_triggerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#disable_statement}.
 * @param ctx the parse tree
 */
fn enter_disable_statement(&mut self, _ctx: &Disable_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#disable_statement}.
 * @param ctx the parse tree
 */
fn exit_disable_statement(&mut self, _ctx: &Disable_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#conditional_statement}.
 * @param ctx the parse tree
 */
fn enter_conditional_statement(&mut self, _ctx: &Conditional_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#conditional_statement}.
 * @param ctx the parse tree
 */
fn exit_conditional_statement(&mut self, _ctx: &Conditional_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unique_priority}.
 * @param ctx the parse tree
 */
fn enter_unique_priority(&mut self, _ctx: &Unique_priorityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unique_priority}.
 * @param ctx the parse tree
 */
fn exit_unique_priority(&mut self, _ctx: &Unique_priorityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cond_predicate}.
 * @param ctx the parse tree
 */
fn enter_cond_predicate(&mut self, _ctx: &Cond_predicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cond_predicate}.
 * @param ctx the parse tree
 */
fn exit_cond_predicate(&mut self, _ctx: &Cond_predicateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#expression_or_cond_pattern}.
 * @param ctx the parse tree
 */
fn enter_expression_or_cond_pattern(&mut self, _ctx: &Expression_or_cond_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#expression_or_cond_pattern}.
 * @param ctx the parse tree
 */
fn exit_expression_or_cond_pattern(&mut self, _ctx: &Expression_or_cond_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_statement}.
 * @param ctx the parse tree
 */
fn enter_case_statement(&mut self, _ctx: &Case_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_statement}.
 * @param ctx the parse tree
 */
fn exit_case_statement(&mut self, _ctx: &Case_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_keyword}.
 * @param ctx the parse tree
 */
fn enter_case_keyword(&mut self, _ctx: &Case_keywordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_keyword}.
 * @param ctx the parse tree
 */
fn exit_case_keyword(&mut self, _ctx: &Case_keywordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_expression}.
 * @param ctx the parse tree
 */
fn enter_case_expression(&mut self, _ctx: &Case_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_expression}.
 * @param ctx the parse tree
 */
fn exit_case_expression(&mut self, _ctx: &Case_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_item}.
 * @param ctx the parse tree
 */
fn enter_case_item(&mut self, _ctx: &Case_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_item}.
 * @param ctx the parse tree
 */
fn exit_case_item(&mut self, _ctx: &Case_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_pattern_item}.
 * @param ctx the parse tree
 */
fn enter_case_pattern_item(&mut self, _ctx: &Case_pattern_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_pattern_item}.
 * @param ctx the parse tree
 */
fn exit_case_pattern_item(&mut self, _ctx: &Case_pattern_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_inside_item}.
 * @param ctx the parse tree
 */
fn enter_case_inside_item(&mut self, _ctx: &Case_inside_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_inside_item}.
 * @param ctx the parse tree
 */
fn exit_case_inside_item(&mut self, _ctx: &Case_inside_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#case_item_expression}.
 * @param ctx the parse tree
 */
fn enter_case_item_expression(&mut self, _ctx: &Case_item_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#case_item_expression}.
 * @param ctx the parse tree
 */
fn exit_case_item_expression(&mut self, _ctx: &Case_item_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#randcase_statement}.
 * @param ctx the parse tree
 */
fn enter_randcase_statement(&mut self, _ctx: &Randcase_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#randcase_statement}.
 * @param ctx the parse tree
 */
fn exit_randcase_statement(&mut self, _ctx: &Randcase_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#randcase_item}.
 * @param ctx the parse tree
 */
fn enter_randcase_item(&mut self, _ctx: &Randcase_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#randcase_item}.
 * @param ctx the parse tree
 */
fn exit_randcase_item(&mut self, _ctx: &Randcase_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#open_range_list}.
 * @param ctx the parse tree
 */
fn enter_open_range_list(&mut self, _ctx: &Open_range_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#open_range_list}.
 * @param ctx the parse tree
 */
fn exit_open_range_list(&mut self, _ctx: &Open_range_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#open_value_range}.
 * @param ctx the parse tree
 */
fn enter_open_value_range(&mut self, _ctx: &Open_value_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#open_value_range}.
 * @param ctx the parse tree
 */
fn exit_open_value_range(&mut self, _ctx: &Open_value_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#member_pattern_pair}.
 * @param ctx the parse tree
 */
fn enter_member_pattern_pair(&mut self, _ctx: &Member_pattern_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#member_pattern_pair}.
 * @param ctx the parse tree
 */
fn exit_member_pattern_pair(&mut self, _ctx: &Member_pattern_pairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_pattern}.
 * @param ctx the parse tree
 */
fn enter_assignment_pattern(&mut self, _ctx: &Assignment_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_pattern}.
 * @param ctx the parse tree
 */
fn exit_assignment_pattern(&mut self, _ctx: &Assignment_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#array_key_val_pair}.
 * @param ctx the parse tree
 */
fn enter_array_key_val_pair(&mut self, _ctx: &Array_key_val_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#array_key_val_pair}.
 * @param ctx the parse tree
 */
fn exit_array_key_val_pair(&mut self, _ctx: &Array_key_val_pairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#array_pattern_key}.
 * @param ctx the parse tree
 */
fn enter_array_pattern_key(&mut self, _ctx: &Array_pattern_keyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#array_pattern_key}.
 * @param ctx the parse tree
 */
fn exit_array_pattern_key(&mut self, _ctx: &Array_pattern_keyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_pattern_key}.
 * @param ctx the parse tree
 */
fn enter_assignment_pattern_key(&mut self, _ctx: &Assignment_pattern_keyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_pattern_key}.
 * @param ctx the parse tree
 */
fn exit_assignment_pattern_key(&mut self, _ctx: &Assignment_pattern_keyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_pattern_expression}.
 * @param ctx the parse tree
 */
fn enter_assignment_pattern_expression(&mut self, _ctx: &Assignment_pattern_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_pattern_expression}.
 * @param ctx the parse tree
 */
fn exit_assignment_pattern_expression(&mut self, _ctx: &Assignment_pattern_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_pattern_expression_type}.
 * @param ctx the parse tree
 */
fn enter_assignment_pattern_expression_type(&mut self, _ctx: &Assignment_pattern_expression_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_pattern_expression_type}.
 * @param ctx the parse tree
 */
fn exit_assignment_pattern_expression_type(&mut self, _ctx: &Assignment_pattern_expression_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_assignment_pattern_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_assignment_pattern_expression(&mut self, _ctx: &Constant_assignment_pattern_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_assignment_pattern_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_assignment_pattern_expression(&mut self, _ctx: &Constant_assignment_pattern_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_pattern_net_lvalue}.
 * @param ctx the parse tree
 */
fn enter_assignment_pattern_net_lvalue(&mut self, _ctx: &Assignment_pattern_net_lvalueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_pattern_net_lvalue}.
 * @param ctx the parse tree
 */
fn exit_assignment_pattern_net_lvalue(&mut self, _ctx: &Assignment_pattern_net_lvalueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assignment_pattern_variable_lvalue}.
 * @param ctx the parse tree
 */
fn enter_assignment_pattern_variable_lvalue(&mut self, _ctx: &Assignment_pattern_variable_lvalueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assignment_pattern_variable_lvalue}.
 * @param ctx the parse tree
 */
fn exit_assignment_pattern_variable_lvalue(&mut self, _ctx: &Assignment_pattern_variable_lvalueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#loop_statement}.
 * @param ctx the parse tree
 */
fn enter_loop_statement(&mut self, _ctx: &Loop_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#loop_statement}.
 * @param ctx the parse tree
 */
fn exit_loop_statement(&mut self, _ctx: &Loop_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#for_initialization}.
 * @param ctx the parse tree
 */
fn enter_for_initialization(&mut self, _ctx: &For_initializationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#for_initialization}.
 * @param ctx the parse tree
 */
fn exit_for_initialization(&mut self, _ctx: &For_initializationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#for_variable_declaration}.
 * @param ctx the parse tree
 */
fn enter_for_variable_declaration(&mut self, _ctx: &For_variable_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#for_variable_declaration}.
 * @param ctx the parse tree
 */
fn exit_for_variable_declaration(&mut self, _ctx: &For_variable_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#for_variable_assign}.
 * @param ctx the parse tree
 */
fn enter_for_variable_assign(&mut self, _ctx: &For_variable_assignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#for_variable_assign}.
 * @param ctx the parse tree
 */
fn exit_for_variable_assign(&mut self, _ctx: &For_variable_assignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#for_step}.
 * @param ctx the parse tree
 */
fn enter_for_step(&mut self, _ctx: &For_stepContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#for_step}.
 * @param ctx the parse tree
 */
fn exit_for_step(&mut self, _ctx: &For_stepContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#for_step_assignment}.
 * @param ctx the parse tree
 */
fn enter_for_step_assignment(&mut self, _ctx: &For_step_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#for_step_assignment}.
 * @param ctx the parse tree
 */
fn exit_for_step_assignment(&mut self, _ctx: &For_step_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#loop_variables}.
 * @param ctx the parse tree
 */
fn enter_loop_variables(&mut self, _ctx: &Loop_variablesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#loop_variables}.
 * @param ctx the parse tree
 */
fn exit_loop_variables(&mut self, _ctx: &Loop_variablesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#loop_var}.
 * @param ctx the parse tree
 */
fn enter_loop_var(&mut self, _ctx: &Loop_varContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#loop_var}.
 * @param ctx the parse tree
 */
fn exit_loop_var(&mut self, _ctx: &Loop_varContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#subroutine_call_statement}.
 * @param ctx the parse tree
 */
fn enter_subroutine_call_statement(&mut self, _ctx: &Subroutine_call_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#subroutine_call_statement}.
 * @param ctx the parse tree
 */
fn exit_subroutine_call_statement(&mut self, _ctx: &Subroutine_call_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#assertion_item}.
 * @param ctx the parse tree
 */
fn enter_assertion_item(&mut self, _ctx: &Assertion_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#assertion_item}.
 * @param ctx the parse tree
 */
fn exit_assertion_item(&mut self, _ctx: &Assertion_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assertion_item}.
 * @param ctx the parse tree
 */
fn enter_deferred_immediate_assertion_item(&mut self, _ctx: &Deferred_immediate_assertion_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assertion_item}.
 * @param ctx the parse tree
 */
fn exit_deferred_immediate_assertion_item(&mut self, _ctx: &Deferred_immediate_assertion_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#procedural_assertion_statement}.
 * @param ctx the parse tree
 */
fn enter_procedural_assertion_statement(&mut self, _ctx: &Procedural_assertion_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#procedural_assertion_statement}.
 * @param ctx the parse tree
 */
fn exit_procedural_assertion_statement(&mut self, _ctx: &Procedural_assertion_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#immediate_assertion_statement}.
 * @param ctx the parse tree
 */
fn enter_immediate_assertion_statement(&mut self, _ctx: &Immediate_assertion_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#immediate_assertion_statement}.
 * @param ctx the parse tree
 */
fn exit_immediate_assertion_statement(&mut self, _ctx: &Immediate_assertion_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_immediate_assertion_statement}.
 * @param ctx the parse tree
 */
fn enter_simple_immediate_assertion_statement(&mut self, _ctx: &Simple_immediate_assertion_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_immediate_assertion_statement}.
 * @param ctx the parse tree
 */
fn exit_simple_immediate_assertion_statement(&mut self, _ctx: &Simple_immediate_assertion_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_immediate_assert_statement}.
 * @param ctx the parse tree
 */
fn enter_simple_immediate_assert_statement(&mut self, _ctx: &Simple_immediate_assert_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_immediate_assert_statement}.
 * @param ctx the parse tree
 */
fn exit_simple_immediate_assert_statement(&mut self, _ctx: &Simple_immediate_assert_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_immediate_assume_statement}.
 * @param ctx the parse tree
 */
fn enter_simple_immediate_assume_statement(&mut self, _ctx: &Simple_immediate_assume_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_immediate_assume_statement}.
 * @param ctx the parse tree
 */
fn exit_simple_immediate_assume_statement(&mut self, _ctx: &Simple_immediate_assume_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_immediate_cover_statement}.
 * @param ctx the parse tree
 */
fn enter_simple_immediate_cover_statement(&mut self, _ctx: &Simple_immediate_cover_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_immediate_cover_statement}.
 * @param ctx the parse tree
 */
fn exit_simple_immediate_cover_statement(&mut self, _ctx: &Simple_immediate_cover_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assertion_statement}.
 * @param ctx the parse tree
 */
fn enter_deferred_immediate_assertion_statement(&mut self, _ctx: &Deferred_immediate_assertion_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assertion_statement}.
 * @param ctx the parse tree
 */
fn exit_deferred_immediate_assertion_statement(&mut self, _ctx: &Deferred_immediate_assertion_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assert_statement}.
 * @param ctx the parse tree
 */
fn enter_deferred_immediate_assert_statement(&mut self, _ctx: &Deferred_immediate_assert_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assert_statement}.
 * @param ctx the parse tree
 */
fn exit_deferred_immediate_assert_statement(&mut self, _ctx: &Deferred_immediate_assert_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assume_statement}.
 * @param ctx the parse tree
 */
fn enter_deferred_immediate_assume_statement(&mut self, _ctx: &Deferred_immediate_assume_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#deferred_immediate_assume_statement}.
 * @param ctx the parse tree
 */
fn exit_deferred_immediate_assume_statement(&mut self, _ctx: &Deferred_immediate_assume_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#deferred_immediate_cover_statement}.
 * @param ctx the parse tree
 */
fn enter_deferred_immediate_cover_statement(&mut self, _ctx: &Deferred_immediate_cover_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#deferred_immediate_cover_statement}.
 * @param ctx the parse tree
 */
fn exit_deferred_immediate_cover_statement(&mut self, _ctx: &Deferred_immediate_cover_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_declaration}.
 * @param ctx the parse tree
 */
fn enter_clocking_declaration(&mut self, _ctx: &Clocking_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_declaration}.
 * @param ctx the parse tree
 */
fn exit_clocking_declaration(&mut self, _ctx: &Clocking_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_name}.
 * @param ctx the parse tree
 */
fn enter_clocking_name(&mut self, _ctx: &Clocking_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_name}.
 * @param ctx the parse tree
 */
fn exit_clocking_name(&mut self, _ctx: &Clocking_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_event}.
 * @param ctx the parse tree
 */
fn enter_clocking_event(&mut self, _ctx: &Clocking_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_event}.
 * @param ctx the parse tree
 */
fn exit_clocking_event(&mut self, _ctx: &Clocking_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_item}.
 * @param ctx the parse tree
 */
fn enter_clocking_item(&mut self, _ctx: &Clocking_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_item}.
 * @param ctx the parse tree
 */
fn exit_clocking_item(&mut self, _ctx: &Clocking_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#default_skew}.
 * @param ctx the parse tree
 */
fn enter_default_skew(&mut self, _ctx: &Default_skewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#default_skew}.
 * @param ctx the parse tree
 */
fn exit_default_skew(&mut self, _ctx: &Default_skewContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_direction}.
 * @param ctx the parse tree
 */
fn enter_clocking_direction(&mut self, _ctx: &Clocking_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_direction}.
 * @param ctx the parse tree
 */
fn exit_clocking_direction(&mut self, _ctx: &Clocking_directionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_clocking_decl_assign}.
 * @param ctx the parse tree
 */
fn enter_list_of_clocking_decl_assign(&mut self, _ctx: &List_of_clocking_decl_assignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_clocking_decl_assign}.
 * @param ctx the parse tree
 */
fn exit_list_of_clocking_decl_assign(&mut self, _ctx: &List_of_clocking_decl_assignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_decl_assign}.
 * @param ctx the parse tree
 */
fn enter_clocking_decl_assign(&mut self, _ctx: &Clocking_decl_assignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_decl_assign}.
 * @param ctx the parse tree
 */
fn exit_clocking_decl_assign(&mut self, _ctx: &Clocking_decl_assignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_skew}.
 * @param ctx the parse tree
 */
fn enter_clocking_skew(&mut self, _ctx: &Clocking_skewContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_skew}.
 * @param ctx the parse tree
 */
fn exit_clocking_skew(&mut self, _ctx: &Clocking_skewContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_drive}.
 * @param ctx the parse tree
 */
fn enter_clocking_drive(&mut self, _ctx: &Clocking_driveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_drive}.
 * @param ctx the parse tree
 */
fn exit_clocking_drive(&mut self, _ctx: &Clocking_driveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cycle_delay}.
 * @param ctx the parse tree
 */
fn enter_cycle_delay(&mut self, _ctx: &Cycle_delayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cycle_delay}.
 * @param ctx the parse tree
 */
fn exit_cycle_delay(&mut self, _ctx: &Cycle_delayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clockvar}.
 * @param ctx the parse tree
 */
fn enter_clockvar(&mut self, _ctx: &ClockvarContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clockvar}.
 * @param ctx the parse tree
 */
fn exit_clockvar(&mut self, _ctx: &ClockvarContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clockvar_expression}.
 * @param ctx the parse tree
 */
fn enter_clockvar_expression(&mut self, _ctx: &Clockvar_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clockvar_expression}.
 * @param ctx the parse tree
 */
fn exit_clockvar_expression(&mut self, _ctx: &Clockvar_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#randsequence_statement}.
 * @param ctx the parse tree
 */
fn enter_randsequence_statement(&mut self, _ctx: &Randsequence_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#randsequence_statement}.
 * @param ctx the parse tree
 */
fn exit_randsequence_statement(&mut self, _ctx: &Randsequence_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#production}.
 * @param ctx the parse tree
 */
fn enter_production(&mut self, _ctx: &ProductionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#production}.
 * @param ctx the parse tree
 */
fn exit_production(&mut self, _ctx: &ProductionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_rule}.
 * @param ctx the parse tree
 */
fn enter_rs_rule(&mut self, _ctx: &Rs_ruleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_rule}.
 * @param ctx the parse tree
 */
fn exit_rs_rule(&mut self, _ctx: &Rs_ruleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#weight_spec}.
 * @param ctx the parse tree
 */
fn enter_weight_spec(&mut self, _ctx: &Weight_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#weight_spec}.
 * @param ctx the parse tree
 */
fn exit_weight_spec(&mut self, _ctx: &Weight_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_production_list}.
 * @param ctx the parse tree
 */
fn enter_rs_production_list(&mut self, _ctx: &Rs_production_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_production_list}.
 * @param ctx the parse tree
 */
fn exit_rs_production_list(&mut self, _ctx: &Rs_production_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#weight_specification}.
 * @param ctx the parse tree
 */
fn enter_weight_specification(&mut self, _ctx: &Weight_specificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#weight_specification}.
 * @param ctx the parse tree
 */
fn exit_weight_specification(&mut self, _ctx: &Weight_specificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_code_block}.
 * @param ctx the parse tree
 */
fn enter_rs_code_block(&mut self, _ctx: &Rs_code_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_code_block}.
 * @param ctx the parse tree
 */
fn exit_rs_code_block(&mut self, _ctx: &Rs_code_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_prod}.
 * @param ctx the parse tree
 */
fn enter_rs_prod(&mut self, _ctx: &Rs_prodContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_prod}.
 * @param ctx the parse tree
 */
fn exit_rs_prod(&mut self, _ctx: &Rs_prodContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#production_item}.
 * @param ctx the parse tree
 */
fn enter_production_item(&mut self, _ctx: &Production_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#production_item}.
 * @param ctx the parse tree
 */
fn exit_production_item(&mut self, _ctx: &Production_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_if_else}.
 * @param ctx the parse tree
 */
fn enter_rs_if_else(&mut self, _ctx: &Rs_if_elseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_if_else}.
 * @param ctx the parse tree
 */
fn exit_rs_if_else(&mut self, _ctx: &Rs_if_elseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_repeat}.
 * @param ctx the parse tree
 */
fn enter_rs_repeat(&mut self, _ctx: &Rs_repeatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_repeat}.
 * @param ctx the parse tree
 */
fn exit_rs_repeat(&mut self, _ctx: &Rs_repeatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_case}.
 * @param ctx the parse tree
 */
fn enter_rs_case(&mut self, _ctx: &Rs_caseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_case}.
 * @param ctx the parse tree
 */
fn exit_rs_case(&mut self, _ctx: &Rs_caseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rs_case_item}.
 * @param ctx the parse tree
 */
fn enter_rs_case_item(&mut self, _ctx: &Rs_case_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rs_case_item}.
 * @param ctx the parse tree
 */
fn exit_rs_case_item(&mut self, _ctx: &Rs_case_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specify_block}.
 * @param ctx the parse tree
 */
fn enter_specify_block(&mut self, _ctx: &Specify_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specify_block}.
 * @param ctx the parse tree
 */
fn exit_specify_block(&mut self, _ctx: &Specify_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specify_item}.
 * @param ctx the parse tree
 */
fn enter_specify_item(&mut self, _ctx: &Specify_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specify_item}.
 * @param ctx the parse tree
 */
fn exit_specify_item(&mut self, _ctx: &Specify_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#pulsestyle_declaration}.
 * @param ctx the parse tree
 */
fn enter_pulsestyle_declaration(&mut self, _ctx: &Pulsestyle_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#pulsestyle_declaration}.
 * @param ctx the parse tree
 */
fn exit_pulsestyle_declaration(&mut self, _ctx: &Pulsestyle_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#showcancelled_declaration}.
 * @param ctx the parse tree
 */
fn enter_showcancelled_declaration(&mut self, _ctx: &Showcancelled_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#showcancelled_declaration}.
 * @param ctx the parse tree
 */
fn exit_showcancelled_declaration(&mut self, _ctx: &Showcancelled_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#path_declaration}.
 * @param ctx the parse tree
 */
fn enter_path_declaration(&mut self, _ctx: &Path_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#path_declaration}.
 * @param ctx the parse tree
 */
fn exit_path_declaration(&mut self, _ctx: &Path_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_path_declaration}.
 * @param ctx the parse tree
 */
fn enter_simple_path_declaration(&mut self, _ctx: &Simple_path_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_path_declaration}.
 * @param ctx the parse tree
 */
fn exit_simple_path_declaration(&mut self, _ctx: &Simple_path_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parallel_path_description}.
 * @param ctx the parse tree
 */
fn enter_parallel_path_description(&mut self, _ctx: &Parallel_path_descriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parallel_path_description}.
 * @param ctx the parse tree
 */
fn exit_parallel_path_description(&mut self, _ctx: &Parallel_path_descriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#full_path_description}.
 * @param ctx the parse tree
 */
fn enter_full_path_description(&mut self, _ctx: &Full_path_descriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#full_path_description}.
 * @param ctx the parse tree
 */
fn exit_full_path_description(&mut self, _ctx: &Full_path_descriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_path_inputs}.
 * @param ctx the parse tree
 */
fn enter_list_of_path_inputs(&mut self, _ctx: &List_of_path_inputsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_path_inputs}.
 * @param ctx the parse tree
 */
fn exit_list_of_path_inputs(&mut self, _ctx: &List_of_path_inputsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_path_outputs}.
 * @param ctx the parse tree
 */
fn enter_list_of_path_outputs(&mut self, _ctx: &List_of_path_outputsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_path_outputs}.
 * @param ctx the parse tree
 */
fn exit_list_of_path_outputs(&mut self, _ctx: &List_of_path_outputsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specify_input_terminal_descriptor}.
 * @param ctx the parse tree
 */
fn enter_specify_input_terminal_descriptor(&mut self, _ctx: &Specify_input_terminal_descriptorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specify_input_terminal_descriptor}.
 * @param ctx the parse tree
 */
fn exit_specify_input_terminal_descriptor(&mut self, _ctx: &Specify_input_terminal_descriptorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specify_output_terminal_descriptor}.
 * @param ctx the parse tree
 */
fn enter_specify_output_terminal_descriptor(&mut self, _ctx: &Specify_output_terminal_descriptorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specify_output_terminal_descriptor}.
 * @param ctx the parse tree
 */
fn exit_specify_output_terminal_descriptor(&mut self, _ctx: &Specify_output_terminal_descriptorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#input_identifier}.
 * @param ctx the parse tree
 */
fn enter_input_identifier(&mut self, _ctx: &Input_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#input_identifier}.
 * @param ctx the parse tree
 */
fn exit_input_identifier(&mut self, _ctx: &Input_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#output_identifier}.
 * @param ctx the parse tree
 */
fn enter_output_identifier(&mut self, _ctx: &Output_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#output_identifier}.
 * @param ctx the parse tree
 */
fn exit_output_identifier(&mut self, _ctx: &Output_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#path_delay_value}.
 * @param ctx the parse tree
 */
fn enter_path_delay_value(&mut self, _ctx: &Path_delay_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#path_delay_value}.
 * @param ctx the parse tree
 */
fn exit_path_delay_value(&mut self, _ctx: &Path_delay_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_path_delay_expressions}.
 * @param ctx the parse tree
 */
fn enter_list_of_path_delay_expressions(&mut self, _ctx: &List_of_path_delay_expressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_path_delay_expressions}.
 * @param ctx the parse tree
 */
fn exit_list_of_path_delay_expressions(&mut self, _ctx: &List_of_path_delay_expressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t_path_delay_expression(&mut self, _ctx: &T_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t_path_delay_expression(&mut self, _ctx: &T_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#trise_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_trise_path_delay_expression(&mut self, _ctx: &Trise_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#trise_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_trise_path_delay_expression(&mut self, _ctx: &Trise_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tfall_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tfall_path_delay_expression(&mut self, _ctx: &Tfall_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tfall_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tfall_path_delay_expression(&mut self, _ctx: &Tfall_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tz_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tz_path_delay_expression(&mut self, _ctx: &Tz_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tz_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tz_path_delay_expression(&mut self, _ctx: &Tz_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t01_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t01_path_delay_expression(&mut self, _ctx: &T01_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t01_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t01_path_delay_expression(&mut self, _ctx: &T01_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t10_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t10_path_delay_expression(&mut self, _ctx: &T10_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t10_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t10_path_delay_expression(&mut self, _ctx: &T10_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t0z_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t0z_path_delay_expression(&mut self, _ctx: &T0z_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t0z_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t0z_path_delay_expression(&mut self, _ctx: &T0z_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tz1_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tz1_path_delay_expression(&mut self, _ctx: &Tz1_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tz1_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tz1_path_delay_expression(&mut self, _ctx: &Tz1_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t1z_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t1z_path_delay_expression(&mut self, _ctx: &T1z_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t1z_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t1z_path_delay_expression(&mut self, _ctx: &T1z_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tz0_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tz0_path_delay_expression(&mut self, _ctx: &Tz0_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tz0_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tz0_path_delay_expression(&mut self, _ctx: &Tz0_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t0x_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t0x_path_delay_expression(&mut self, _ctx: &T0x_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t0x_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t0x_path_delay_expression(&mut self, _ctx: &T0x_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tx1_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tx1_path_delay_expression(&mut self, _ctx: &Tx1_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tx1_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tx1_path_delay_expression(&mut self, _ctx: &Tx1_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#t1x_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_t1x_path_delay_expression(&mut self, _ctx: &T1x_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#t1x_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_t1x_path_delay_expression(&mut self, _ctx: &T1x_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tx0_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tx0_path_delay_expression(&mut self, _ctx: &Tx0_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tx0_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tx0_path_delay_expression(&mut self, _ctx: &Tx0_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#txz_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_txz_path_delay_expression(&mut self, _ctx: &Txz_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#txz_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_txz_path_delay_expression(&mut self, _ctx: &Txz_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tzx_path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_tzx_path_delay_expression(&mut self, _ctx: &Tzx_path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tzx_path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_tzx_path_delay_expression(&mut self, _ctx: &Tzx_path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#path_delay_expression}.
 * @param ctx the parse tree
 */
fn enter_path_delay_expression(&mut self, _ctx: &Path_delay_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#path_delay_expression}.
 * @param ctx the parse tree
 */
fn exit_path_delay_expression(&mut self, _ctx: &Path_delay_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_sensitive_path_declaration}.
 * @param ctx the parse tree
 */
fn enter_edge_sensitive_path_declaration(&mut self, _ctx: &Edge_sensitive_path_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_sensitive_path_declaration}.
 * @param ctx the parse tree
 */
fn exit_edge_sensitive_path_declaration(&mut self, _ctx: &Edge_sensitive_path_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parallel_edge_sensitive_path_description}.
 * @param ctx the parse tree
 */
fn enter_parallel_edge_sensitive_path_description(&mut self, _ctx: &Parallel_edge_sensitive_path_descriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parallel_edge_sensitive_path_description}.
 * @param ctx the parse tree
 */
fn exit_parallel_edge_sensitive_path_description(&mut self, _ctx: &Parallel_edge_sensitive_path_descriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#full_edge_sensitive_path_description}.
 * @param ctx the parse tree
 */
fn enter_full_edge_sensitive_path_description(&mut self, _ctx: &Full_edge_sensitive_path_descriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#full_edge_sensitive_path_description}.
 * @param ctx the parse tree
 */
fn exit_full_edge_sensitive_path_description(&mut self, _ctx: &Full_edge_sensitive_path_descriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#data_source_expression}.
 * @param ctx the parse tree
 */
fn enter_data_source_expression(&mut self, _ctx: &Data_source_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#data_source_expression}.
 * @param ctx the parse tree
 */
fn exit_data_source_expression(&mut self, _ctx: &Data_source_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_identifier}.
 * @param ctx the parse tree
 */
fn enter_edge_identifier(&mut self, _ctx: &Edge_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_identifier}.
 * @param ctx the parse tree
 */
fn exit_edge_identifier(&mut self, _ctx: &Edge_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#state_dependent_path_declaration}.
 * @param ctx the parse tree
 */
fn enter_state_dependent_path_declaration(&mut self, _ctx: &State_dependent_path_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#state_dependent_path_declaration}.
 * @param ctx the parse tree
 */
fn exit_state_dependent_path_declaration(&mut self, _ctx: &State_dependent_path_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#polarity_operator}.
 * @param ctx the parse tree
 */
fn enter_polarity_operator(&mut self, _ctx: &Polarity_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#polarity_operator}.
 * @param ctx the parse tree
 */
fn exit_polarity_operator(&mut self, _ctx: &Polarity_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#system_timing_check}.
 * @param ctx the parse tree
 */
fn enter_system_timing_check(&mut self, _ctx: &System_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#system_timing_check}.
 * @param ctx the parse tree
 */
fn exit_system_timing_check(&mut self, _ctx: &System_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#setup_timing_check}.
 * @param ctx the parse tree
 */
fn enter_setup_timing_check(&mut self, _ctx: &Setup_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#setup_timing_check}.
 * @param ctx the parse tree
 */
fn exit_setup_timing_check(&mut self, _ctx: &Setup_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#notifier_opt}.
 * @param ctx the parse tree
 */
fn enter_notifier_opt(&mut self, _ctx: &Notifier_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#notifier_opt}.
 * @param ctx the parse tree
 */
fn exit_notifier_opt(&mut self, _ctx: &Notifier_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hold_timing_check}.
 * @param ctx the parse tree
 */
fn enter_hold_timing_check(&mut self, _ctx: &Hold_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hold_timing_check}.
 * @param ctx the parse tree
 */
fn exit_hold_timing_check(&mut self, _ctx: &Hold_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#setuphold_timing_check}.
 * @param ctx the parse tree
 */
fn enter_setuphold_timing_check(&mut self, _ctx: &Setuphold_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#setuphold_timing_check}.
 * @param ctx the parse tree
 */
fn exit_setuphold_timing_check(&mut self, _ctx: &Setuphold_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timing_check_opt}.
 * @param ctx the parse tree
 */
fn enter_timing_check_opt(&mut self, _ctx: &Timing_check_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timing_check_opt}.
 * @param ctx the parse tree
 */
fn exit_timing_check_opt(&mut self, _ctx: &Timing_check_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timestamp_cond_opt}.
 * @param ctx the parse tree
 */
fn enter_timestamp_cond_opt(&mut self, _ctx: &Timestamp_cond_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timestamp_cond_opt}.
 * @param ctx the parse tree
 */
fn exit_timestamp_cond_opt(&mut self, _ctx: &Timestamp_cond_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timecheck_cond_opt}.
 * @param ctx the parse tree
 */
fn enter_timecheck_cond_opt(&mut self, _ctx: &Timecheck_cond_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timecheck_cond_opt}.
 * @param ctx the parse tree
 */
fn exit_timecheck_cond_opt(&mut self, _ctx: &Timecheck_cond_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delayed_ref_opt}.
 * @param ctx the parse tree
 */
fn enter_delayed_ref_opt(&mut self, _ctx: &Delayed_ref_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delayed_ref_opt}.
 * @param ctx the parse tree
 */
fn exit_delayed_ref_opt(&mut self, _ctx: &Delayed_ref_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delayed_data_opt}.
 * @param ctx the parse tree
 */
fn enter_delayed_data_opt(&mut self, _ctx: &Delayed_data_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delayed_data_opt}.
 * @param ctx the parse tree
 */
fn exit_delayed_data_opt(&mut self, _ctx: &Delayed_data_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#recovery_timing_check}.
 * @param ctx the parse tree
 */
fn enter_recovery_timing_check(&mut self, _ctx: &Recovery_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#recovery_timing_check}.
 * @param ctx the parse tree
 */
fn exit_recovery_timing_check(&mut self, _ctx: &Recovery_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#removal_timing_check}.
 * @param ctx the parse tree
 */
fn enter_removal_timing_check(&mut self, _ctx: &Removal_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#removal_timing_check}.
 * @param ctx the parse tree
 */
fn exit_removal_timing_check(&mut self, _ctx: &Removal_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#recrem_timing_check}.
 * @param ctx the parse tree
 */
fn enter_recrem_timing_check(&mut self, _ctx: &Recrem_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#recrem_timing_check}.
 * @param ctx the parse tree
 */
fn exit_recrem_timing_check(&mut self, _ctx: &Recrem_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#skew_timing_check}.
 * @param ctx the parse tree
 */
fn enter_skew_timing_check(&mut self, _ctx: &Skew_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#skew_timing_check}.
 * @param ctx the parse tree
 */
fn exit_skew_timing_check(&mut self, _ctx: &Skew_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timeskew_timing_check}.
 * @param ctx the parse tree
 */
fn enter_timeskew_timing_check(&mut self, _ctx: &Timeskew_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timeskew_timing_check}.
 * @param ctx the parse tree
 */
fn exit_timeskew_timing_check(&mut self, _ctx: &Timeskew_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#skew_timing_check_opt}.
 * @param ctx the parse tree
 */
fn enter_skew_timing_check_opt(&mut self, _ctx: &Skew_timing_check_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#skew_timing_check_opt}.
 * @param ctx the parse tree
 */
fn exit_skew_timing_check_opt(&mut self, _ctx: &Skew_timing_check_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#event_based_flag_opt}.
 * @param ctx the parse tree
 */
fn enter_event_based_flag_opt(&mut self, _ctx: &Event_based_flag_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#event_based_flag_opt}.
 * @param ctx the parse tree
 */
fn exit_event_based_flag_opt(&mut self, _ctx: &Event_based_flag_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#remain_active_flag_opt}.
 * @param ctx the parse tree
 */
fn enter_remain_active_flag_opt(&mut self, _ctx: &Remain_active_flag_optContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#remain_active_flag_opt}.
 * @param ctx the parse tree
 */
fn exit_remain_active_flag_opt(&mut self, _ctx: &Remain_active_flag_optContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#fullskew_timing_check}.
 * @param ctx the parse tree
 */
fn enter_fullskew_timing_check(&mut self, _ctx: &Fullskew_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#fullskew_timing_check}.
 * @param ctx the parse tree
 */
fn exit_fullskew_timing_check(&mut self, _ctx: &Fullskew_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#period_timing_check}.
 * @param ctx the parse tree
 */
fn enter_period_timing_check(&mut self, _ctx: &Period_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#period_timing_check}.
 * @param ctx the parse tree
 */
fn exit_period_timing_check(&mut self, _ctx: &Period_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#width_timing_check}.
 * @param ctx the parse tree
 */
fn enter_width_timing_check(&mut self, _ctx: &Width_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#width_timing_check}.
 * @param ctx the parse tree
 */
fn exit_width_timing_check(&mut self, _ctx: &Width_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#nochange_timing_check}.
 * @param ctx the parse tree
 */
fn enter_nochange_timing_check(&mut self, _ctx: &Nochange_timing_checkContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#nochange_timing_check}.
 * @param ctx the parse tree
 */
fn exit_nochange_timing_check(&mut self, _ctx: &Nochange_timing_checkContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timecheck_condition}.
 * @param ctx the parse tree
 */
fn enter_timecheck_condition(&mut self, _ctx: &Timecheck_conditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timecheck_condition}.
 * @param ctx the parse tree
 */
fn exit_timecheck_condition(&mut self, _ctx: &Timecheck_conditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#controlled_reference_event}.
 * @param ctx the parse tree
 */
fn enter_controlled_reference_event(&mut self, _ctx: &Controlled_reference_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#controlled_reference_event}.
 * @param ctx the parse tree
 */
fn exit_controlled_reference_event(&mut self, _ctx: &Controlled_reference_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#data_event}.
 * @param ctx the parse tree
 */
fn enter_data_event(&mut self, _ctx: &Data_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#data_event}.
 * @param ctx the parse tree
 */
fn exit_data_event(&mut self, _ctx: &Data_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delayed_data}.
 * @param ctx the parse tree
 */
fn enter_delayed_data(&mut self, _ctx: &Delayed_dataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delayed_data}.
 * @param ctx the parse tree
 */
fn exit_delayed_data(&mut self, _ctx: &Delayed_dataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#delayed_reference}.
 * @param ctx the parse tree
 */
fn enter_delayed_reference(&mut self, _ctx: &Delayed_referenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#delayed_reference}.
 * @param ctx the parse tree
 */
fn exit_delayed_reference(&mut self, _ctx: &Delayed_referenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#end_edge_offset}.
 * @param ctx the parse tree
 */
fn enter_end_edge_offset(&mut self, _ctx: &End_edge_offsetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#end_edge_offset}.
 * @param ctx the parse tree
 */
fn exit_end_edge_offset(&mut self, _ctx: &End_edge_offsetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#event_based_flag}.
 * @param ctx the parse tree
 */
fn enter_event_based_flag(&mut self, _ctx: &Event_based_flagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#event_based_flag}.
 * @param ctx the parse tree
 */
fn exit_event_based_flag(&mut self, _ctx: &Event_based_flagContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#notifier}.
 * @param ctx the parse tree
 */
fn enter_notifier(&mut self, _ctx: &NotifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#notifier}.
 * @param ctx the parse tree
 */
fn exit_notifier(&mut self, _ctx: &NotifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#reference_event}.
 * @param ctx the parse tree
 */
fn enter_reference_event(&mut self, _ctx: &Reference_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#reference_event}.
 * @param ctx the parse tree
 */
fn exit_reference_event(&mut self, _ctx: &Reference_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#remain_active_flag}.
 * @param ctx the parse tree
 */
fn enter_remain_active_flag(&mut self, _ctx: &Remain_active_flagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#remain_active_flag}.
 * @param ctx the parse tree
 */
fn exit_remain_active_flag(&mut self, _ctx: &Remain_active_flagContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timestamp_condition}.
 * @param ctx the parse tree
 */
fn enter_timestamp_condition(&mut self, _ctx: &Timestamp_conditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timestamp_condition}.
 * @param ctx the parse tree
 */
fn exit_timestamp_condition(&mut self, _ctx: &Timestamp_conditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#start_edge_offset}.
 * @param ctx the parse tree
 */
fn enter_start_edge_offset(&mut self, _ctx: &Start_edge_offsetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#start_edge_offset}.
 * @param ctx the parse tree
 */
fn exit_start_edge_offset(&mut self, _ctx: &Start_edge_offsetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#threshold}.
 * @param ctx the parse tree
 */
fn enter_threshold(&mut self, _ctx: &ThresholdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#threshold}.
 * @param ctx the parse tree
 */
fn exit_threshold(&mut self, _ctx: &ThresholdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timing_check_limit}.
 * @param ctx the parse tree
 */
fn enter_timing_check_limit(&mut self, _ctx: &Timing_check_limitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timing_check_limit}.
 * @param ctx the parse tree
 */
fn exit_timing_check_limit(&mut self, _ctx: &Timing_check_limitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timing_check_event}.
 * @param ctx the parse tree
 */
fn enter_timing_check_event(&mut self, _ctx: &Timing_check_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timing_check_event}.
 * @param ctx the parse tree
 */
fn exit_timing_check_event(&mut self, _ctx: &Timing_check_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#controlled_timing_check_event}.
 * @param ctx the parse tree
 */
fn enter_controlled_timing_check_event(&mut self, _ctx: &Controlled_timing_check_eventContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#controlled_timing_check_event}.
 * @param ctx the parse tree
 */
fn exit_controlled_timing_check_event(&mut self, _ctx: &Controlled_timing_check_eventContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timing_check_event_control}.
 * @param ctx the parse tree
 */
fn enter_timing_check_event_control(&mut self, _ctx: &Timing_check_event_controlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timing_check_event_control}.
 * @param ctx the parse tree
 */
fn exit_timing_check_event_control(&mut self, _ctx: &Timing_check_event_controlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specify_terminal_descriptor}.
 * @param ctx the parse tree
 */
fn enter_specify_terminal_descriptor(&mut self, _ctx: &Specify_terminal_descriptorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specify_terminal_descriptor}.
 * @param ctx the parse tree
 */
fn exit_specify_terminal_descriptor(&mut self, _ctx: &Specify_terminal_descriptorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_control_specifier}.
 * @param ctx the parse tree
 */
fn enter_edge_control_specifier(&mut self, _ctx: &Edge_control_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_control_specifier}.
 * @param ctx the parse tree
 */
fn exit_edge_control_specifier(&mut self, _ctx: &Edge_control_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#edge_descriptor}.
 * @param ctx the parse tree
 */
fn enter_edge_descriptor(&mut self, _ctx: &Edge_descriptorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#edge_descriptor}.
 * @param ctx the parse tree
 */
fn exit_edge_descriptor(&mut self, _ctx: &Edge_descriptorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#timing_check_condition}.
 * @param ctx the parse tree
 */
fn enter_timing_check_condition(&mut self, _ctx: &Timing_check_conditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#timing_check_condition}.
 * @param ctx the parse tree
 */
fn exit_timing_check_condition(&mut self, _ctx: &Timing_check_conditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#scalar_timing_check_condition}.
 * @param ctx the parse tree
 */
fn enter_scalar_timing_check_condition(&mut self, _ctx: &Scalar_timing_check_conditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#scalar_timing_check_condition}.
 * @param ctx the parse tree
 */
fn exit_scalar_timing_check_condition(&mut self, _ctx: &Scalar_timing_check_conditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#scalar_constant}.
 * @param ctx the parse tree
 */
fn enter_scalar_constant(&mut self, _ctx: &Scalar_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#scalar_constant}.
 * @param ctx the parse tree
 */
fn exit_scalar_constant(&mut self, _ctx: &Scalar_constantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#concatenation}.
 * @param ctx the parse tree
 */
fn enter_concatenation(&mut self, _ctx: &ConcatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#concatenation}.
 * @param ctx the parse tree
 */
fn exit_concatenation(&mut self, _ctx: &ConcatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_concatenation}.
 * @param ctx the parse tree
 */
fn enter_constant_concatenation(&mut self, _ctx: &Constant_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_concatenation}.
 * @param ctx the parse tree
 */
fn exit_constant_concatenation(&mut self, _ctx: &Constant_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_multiple_concatenation}.
 * @param ctx the parse tree
 */
fn enter_constant_multiple_concatenation(&mut self, _ctx: &Constant_multiple_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_multiple_concatenation}.
 * @param ctx the parse tree
 */
fn exit_constant_multiple_concatenation(&mut self, _ctx: &Constant_multiple_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_path_concatenation}.
 * @param ctx the parse tree
 */
fn enter_module_path_concatenation(&mut self, _ctx: &Module_path_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_path_concatenation}.
 * @param ctx the parse tree
 */
fn exit_module_path_concatenation(&mut self, _ctx: &Module_path_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_path_multiple_concatenation}.
 * @param ctx the parse tree
 */
fn enter_module_path_multiple_concatenation(&mut self, _ctx: &Module_path_multiple_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_path_multiple_concatenation}.
 * @param ctx the parse tree
 */
fn exit_module_path_multiple_concatenation(&mut self, _ctx: &Module_path_multiple_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#multiple_concatenation}.
 * @param ctx the parse tree
 */
fn enter_multiple_concatenation(&mut self, _ctx: &Multiple_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#multiple_concatenation}.
 * @param ctx the parse tree
 */
fn exit_multiple_concatenation(&mut self, _ctx: &Multiple_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#streaming_concatenation}.
 * @param ctx the parse tree
 */
fn enter_streaming_concatenation(&mut self, _ctx: &Streaming_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#streaming_concatenation}.
 * @param ctx the parse tree
 */
fn exit_streaming_concatenation(&mut self, _ctx: &Streaming_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#stream_operator}.
 * @param ctx the parse tree
 */
fn enter_stream_operator(&mut self, _ctx: &Stream_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#stream_operator}.
 * @param ctx the parse tree
 */
fn exit_stream_operator(&mut self, _ctx: &Stream_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#slice_size}.
 * @param ctx the parse tree
 */
fn enter_slice_size(&mut self, _ctx: &Slice_sizeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#slice_size}.
 * @param ctx the parse tree
 */
fn exit_slice_size(&mut self, _ctx: &Slice_sizeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#stream_concatenation}.
 * @param ctx the parse tree
 */
fn enter_stream_concatenation(&mut self, _ctx: &Stream_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#stream_concatenation}.
 * @param ctx the parse tree
 */
fn exit_stream_concatenation(&mut self, _ctx: &Stream_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#stream_expression}.
 * @param ctx the parse tree
 */
fn enter_stream_expression(&mut self, _ctx: &Stream_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#stream_expression}.
 * @param ctx the parse tree
 */
fn exit_stream_expression(&mut self, _ctx: &Stream_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#array_range_expression}.
 * @param ctx the parse tree
 */
fn enter_array_range_expression(&mut self, _ctx: &Array_range_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#array_range_expression}.
 * @param ctx the parse tree
 */
fn exit_array_range_expression(&mut self, _ctx: &Array_range_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#empty_unpacked_array_concatenation}.
 * @param ctx the parse tree
 */
fn enter_empty_unpacked_array_concatenation(&mut self, _ctx: &Empty_unpacked_array_concatenationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#empty_unpacked_array_concatenation}.
 * @param ctx the parse tree
 */
fn exit_empty_unpacked_array_concatenation(&mut self, _ctx: &Empty_unpacked_array_concatenationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#system_tf_call}.
 * @param ctx the parse tree
 */
fn enter_system_tf_call(&mut self, _ctx: &System_tf_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#system_tf_call}.
 * @param ctx the parse tree
 */
fn exit_system_tf_call(&mut self, _ctx: &System_tf_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#arg_list}.
 * @param ctx the parse tree
 */
fn enter_arg_list(&mut self, _ctx: &Arg_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#arg_list}.
 * @param ctx the parse tree
 */
fn exit_arg_list(&mut self, _ctx: &Arg_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#subroutine_call}.
 * @param ctx the parse tree
 */
fn enter_subroutine_call(&mut self, _ctx: &Subroutine_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#subroutine_call}.
 * @param ctx the parse tree
 */
fn exit_subroutine_call(&mut self, _ctx: &Subroutine_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#list_of_arguments}.
 * @param ctx the parse tree
 */
fn enter_list_of_arguments(&mut self, _ctx: &List_of_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#list_of_arguments}.
 * @param ctx the parse tree
 */
fn exit_list_of_arguments(&mut self, _ctx: &List_of_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ordered_arg}.
 * @param ctx the parse tree
 */
fn enter_ordered_arg(&mut self, _ctx: &Ordered_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ordered_arg}.
 * @param ctx the parse tree
 */
fn exit_ordered_arg(&mut self, _ctx: &Ordered_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#named_arg}.
 * @param ctx the parse tree
 */
fn enter_named_arg(&mut self, _ctx: &Named_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#named_arg}.
 * @param ctx the parse tree
 */
fn exit_named_arg(&mut self, _ctx: &Named_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#array_manipulation_call}.
 * @param ctx the parse tree
 */
fn enter_array_manipulation_call(&mut self, _ctx: &Array_manipulation_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#array_manipulation_call}.
 * @param ctx the parse tree
 */
fn exit_array_manipulation_call(&mut self, _ctx: &Array_manipulation_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#randomize_call}.
 * @param ctx the parse tree
 */
fn enter_randomize_call(&mut self, _ctx: &Randomize_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#randomize_call}.
 * @param ctx the parse tree
 */
fn exit_randomize_call(&mut self, _ctx: &Randomize_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rand_list}.
 * @param ctx the parse tree
 */
fn enter_rand_list(&mut self, _ctx: &Rand_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rand_list}.
 * @param ctx the parse tree
 */
fn exit_rand_list(&mut self, _ctx: &Rand_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#rand_with}.
 * @param ctx the parse tree
 */
fn enter_rand_with(&mut self, _ctx: &Rand_withContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#rand_with}.
 * @param ctx the parse tree
 */
fn exit_rand_with(&mut self, _ctx: &Rand_withContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#id_list}.
 * @param ctx the parse tree
 */
fn enter_id_list(&mut self, _ctx: &Id_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#id_list}.
 * @param ctx the parse tree
 */
fn exit_id_list(&mut self, _ctx: &Id_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#method_call_root}.
 * @param ctx the parse tree
 */
fn enter_method_call_root(&mut self, _ctx: &Method_call_rootContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#method_call_root}.
 * @param ctx the parse tree
 */
fn exit_method_call_root(&mut self, _ctx: &Method_call_rootContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#array_method_name}.
 * @param ctx the parse tree
 */
fn enter_array_method_name(&mut self, _ctx: &Array_method_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#array_method_name}.
 * @param ctx the parse tree
 */
fn exit_array_method_name(&mut self, _ctx: &Array_method_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#inc_or_dec_expression}.
 * @param ctx the parse tree
 */
fn enter_inc_or_dec_expression(&mut self, _ctx: &Inc_or_dec_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#inc_or_dec_expression}.
 * @param ctx the parse tree
 */
fn exit_inc_or_dec_expression(&mut self, _ctx: &Inc_or_dec_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_expression(&mut self, _ctx: &Constant_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_expression(&mut self, _ctx: &Constant_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_mintypmax_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_mintypmax_expression(&mut self, _ctx: &Constant_mintypmax_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_mintypmax_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_mintypmax_expression(&mut self, _ctx: &Constant_mintypmax_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_param_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_param_expression(&mut self, _ctx: &Constant_param_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_param_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_param_expression(&mut self, _ctx: &Constant_param_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#param_expression}.
 * @param ctx the parse tree
 */
fn enter_param_expression(&mut self, _ctx: &Param_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#param_expression}.
 * @param ctx the parse tree
 */
fn exit_param_expression(&mut self, _ctx: &Param_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_range_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_range_expression(&mut self, _ctx: &Constant_range_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_range_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_range_expression(&mut self, _ctx: &Constant_range_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_part_select_range}.
 * @param ctx the parse tree
 */
fn enter_constant_part_select_range(&mut self, _ctx: &Constant_part_select_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_part_select_range}.
 * @param ctx the parse tree
 */
fn exit_constant_part_select_range(&mut self, _ctx: &Constant_part_select_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_range}.
 * @param ctx the parse tree
 */
fn enter_constant_range(&mut self, _ctx: &Constant_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_range}.
 * @param ctx the parse tree
 */
fn exit_constant_range(&mut self, _ctx: &Constant_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_indexed_range}.
 * @param ctx the parse tree
 */
fn enter_constant_indexed_range(&mut self, _ctx: &Constant_indexed_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_indexed_range}.
 * @param ctx the parse tree
 */
fn exit_constant_indexed_range(&mut self, _ctx: &Constant_indexed_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tagged_union_expression}.
 * @param ctx the parse tree
 */
fn enter_tagged_union_expression(&mut self, _ctx: &Tagged_union_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tagged_union_expression}.
 * @param ctx the parse tree
 */
fn exit_tagged_union_expression(&mut self, _ctx: &Tagged_union_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#value_range}.
 * @param ctx the parse tree
 */
fn enter_value_range(&mut self, _ctx: &Value_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#value_range}.
 * @param ctx the parse tree
 */
fn exit_value_range(&mut self, _ctx: &Value_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#mintypmax_expression}.
 * @param ctx the parse tree
 */
fn enter_mintypmax_expression(&mut self, _ctx: &Mintypmax_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#mintypmax_expression}.
 * @param ctx the parse tree
 */
fn exit_mintypmax_expression(&mut self, _ctx: &Mintypmax_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_path_expression}.
 * @param ctx the parse tree
 */
fn enter_module_path_expression(&mut self, _ctx: &Module_path_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_path_expression}.
 * @param ctx the parse tree
 */
fn exit_module_path_expression(&mut self, _ctx: &Module_path_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_path_mintypmax_expression}.
 * @param ctx the parse tree
 */
fn enter_module_path_mintypmax_expression(&mut self, _ctx: &Module_path_mintypmax_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_path_mintypmax_expression}.
 * @param ctx the parse tree
 */
fn exit_module_path_mintypmax_expression(&mut self, _ctx: &Module_path_mintypmax_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#part_select_range}.
 * @param ctx the parse tree
 */
fn enter_part_select_range(&mut self, _ctx: &Part_select_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#part_select_range}.
 * @param ctx the parse tree
 */
fn exit_part_select_range(&mut self, _ctx: &Part_select_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#indexed_range}.
 * @param ctx the parse tree
 */
fn enter_indexed_range(&mut self, _ctx: &Indexed_rangeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#indexed_range}.
 * @param ctx the parse tree
 */
fn exit_indexed_range(&mut self, _ctx: &Indexed_rangeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#genvar_expression}.
 * @param ctx the parse tree
 */
fn enter_genvar_expression(&mut self, _ctx: &Genvar_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#genvar_expression}.
 * @param ctx the parse tree
 */
fn exit_genvar_expression(&mut self, _ctx: &Genvar_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_primary}.
 * @param ctx the parse tree
 */
fn enter_constant_primary(&mut self, _ctx: &Constant_primaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_primary}.
 * @param ctx the parse tree
 */
fn exit_constant_primary(&mut self, _ctx: &Constant_primaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_path_primary}.
 * @param ctx the parse tree
 */
fn enter_module_path_primary(&mut self, _ctx: &Module_path_primaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_path_primary}.
 * @param ctx the parse tree
 */
fn exit_module_path_primary(&mut self, _ctx: &Module_path_primaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#range_expression}.
 * @param ctx the parse tree
 */
fn enter_range_expression(&mut self, _ctx: &Range_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#range_expression}.
 * @param ctx the parse tree
 */
fn exit_range_expression(&mut self, _ctx: &Range_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#primary_literal}.
 * @param ctx the parse tree
 */
fn enter_primary_literal(&mut self, _ctx: &Primary_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#primary_literal}.
 * @param ctx the parse tree
 */
fn exit_primary_literal(&mut self, _ctx: &Primary_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#time_literal}.
 * @param ctx the parse tree
 */
fn enter_time_literal(&mut self, _ctx: &Time_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#time_literal}.
 * @param ctx the parse tree
 */
fn exit_time_literal(&mut self, _ctx: &Time_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#implicit_class_handle}.
 * @param ctx the parse tree
 */
fn enter_implicit_class_handle(&mut self, _ctx: &Implicit_class_handleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#implicit_class_handle}.
 * @param ctx the parse tree
 */
fn exit_implicit_class_handle(&mut self, _ctx: &Implicit_class_handleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bit_select}.
 * @param ctx the parse tree
 */
fn enter_bit_select(&mut self, _ctx: &Bit_selectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bit_select}.
 * @param ctx the parse tree
 */
fn exit_bit_select(&mut self, _ctx: &Bit_selectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#select_}.
 * @param ctx the parse tree
 */
fn enter_select_(&mut self, _ctx: &Select_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#select_}.
 * @param ctx the parse tree
 */
fn exit_select_(&mut self, _ctx: &Select_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#nonrange_select}.
 * @param ctx the parse tree
 */
fn enter_nonrange_select(&mut self, _ctx: &Nonrange_selectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#nonrange_select}.
 * @param ctx the parse tree
 */
fn exit_nonrange_select(&mut self, _ctx: &Nonrange_selectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#member_select}.
 * @param ctx the parse tree
 */
fn enter_member_select(&mut self, _ctx: &Member_selectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#member_select}.
 * @param ctx the parse tree
 */
fn exit_member_select(&mut self, _ctx: &Member_selectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_bit_select}.
 * @param ctx the parse tree
 */
fn enter_constant_bit_select(&mut self, _ctx: &Constant_bit_selectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_bit_select}.
 * @param ctx the parse tree
 */
fn exit_constant_bit_select(&mut self, _ctx: &Constant_bit_selectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constant_select}.
 * @param ctx the parse tree
 */
fn enter_constant_select(&mut self, _ctx: &Constant_selectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constant_select}.
 * @param ctx the parse tree
 */
fn exit_constant_select(&mut self, _ctx: &Constant_selectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#const_member_select}.
 * @param ctx the parse tree
 */
fn enter_const_member_select(&mut self, _ctx: &Const_member_selectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#const_member_select}.
 * @param ctx the parse tree
 */
fn exit_const_member_select(&mut self, _ctx: &Const_member_selectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_lvalue}.
 * @param ctx the parse tree
 */
fn enter_net_lvalue(&mut self, _ctx: &Net_lvalueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_lvalue}.
 * @param ctx the parse tree
 */
fn exit_net_lvalue(&mut self, _ctx: &Net_lvalueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_lvalue}.
 * @param ctx the parse tree
 */
fn enter_variable_lvalue(&mut self, _ctx: &Variable_lvalueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_lvalue}.
 * @param ctx the parse tree
 */
fn exit_variable_lvalue(&mut self, _ctx: &Variable_lvalueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#nonrange_variable_lvalue}.
 * @param ctx the parse tree
 */
fn enter_nonrange_variable_lvalue(&mut self, _ctx: &Nonrange_variable_lvalueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#nonrange_variable_lvalue}.
 * @param ctx the parse tree
 */
fn exit_nonrange_variable_lvalue(&mut self, _ctx: &Nonrange_variable_lvalueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unary_operator}.
 * @param ctx the parse tree
 */
fn enter_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unary_operator}.
 * @param ctx the parse tree
 */
fn exit_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#inc_or_dec_operator}.
 * @param ctx the parse tree
 */
fn enter_inc_or_dec_operator(&mut self, _ctx: &Inc_or_dec_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#inc_or_dec_operator}.
 * @param ctx the parse tree
 */
fn exit_inc_or_dec_operator(&mut self, _ctx: &Inc_or_dec_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unary_module_path_operator}.
 * @param ctx the parse tree
 */
fn enter_unary_module_path_operator(&mut self, _ctx: &Unary_module_path_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unary_module_path_operator}.
 * @param ctx the parse tree
 */
fn exit_unary_module_path_operator(&mut self, _ctx: &Unary_module_path_operatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#number}.
 * @param ctx the parse tree
 */
fn enter_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#number}.
 * @param ctx the parse tree
 */
fn exit_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#integral_number}.
 * @param ctx the parse tree
 */
fn enter_integral_number(&mut self, _ctx: &Integral_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#integral_number}.
 * @param ctx the parse tree
 */
fn exit_integral_number(&mut self, _ctx: &Integral_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#decimal_number}.
 * @param ctx the parse tree
 */
fn enter_decimal_number(&mut self, _ctx: &Decimal_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#decimal_number}.
 * @param ctx the parse tree
 */
fn exit_decimal_number(&mut self, _ctx: &Decimal_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#binary_number}.
 * @param ctx the parse tree
 */
fn enter_binary_number(&mut self, _ctx: &Binary_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#binary_number}.
 * @param ctx the parse tree
 */
fn exit_binary_number(&mut self, _ctx: &Binary_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#octal_number}.
 * @param ctx the parse tree
 */
fn enter_octal_number(&mut self, _ctx: &Octal_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#octal_number}.
 * @param ctx the parse tree
 */
fn exit_octal_number(&mut self, _ctx: &Octal_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hex_number}.
 * @param ctx the parse tree
 */
fn enter_hex_number(&mut self, _ctx: &Hex_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hex_number}.
 * @param ctx the parse tree
 */
fn exit_hex_number(&mut self, _ctx: &Hex_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#size}.
 * @param ctx the parse tree
 */
fn enter_size(&mut self, _ctx: &SizeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#size}.
 * @param ctx the parse tree
 */
fn exit_size(&mut self, _ctx: &SizeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#real_number}.
 * @param ctx the parse tree
 */
fn enter_real_number(&mut self, _ctx: &Real_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#real_number}.
 * @param ctx the parse tree
 */
fn exit_real_number(&mut self, _ctx: &Real_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#fixed_point_number}.
 * @param ctx the parse tree
 */
fn enter_fixed_point_number(&mut self, _ctx: &Fixed_point_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#fixed_point_number}.
 * @param ctx the parse tree
 */
fn exit_fixed_point_number(&mut self, _ctx: &Fixed_point_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#exponential_number}.
 * @param ctx the parse tree
 */
fn enter_exponential_number(&mut self, _ctx: &Exponential_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#exponential_number}.
 * @param ctx the parse tree
 */
fn exit_exponential_number(&mut self, _ctx: &Exponential_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unsigned_number}.
 * @param ctx the parse tree
 */
fn enter_unsigned_number(&mut self, _ctx: &Unsigned_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unsigned_number}.
 * @param ctx the parse tree
 */
fn exit_unsigned_number(&mut self, _ctx: &Unsigned_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#decimal_value}.
 * @param ctx the parse tree
 */
fn enter_decimal_value(&mut self, _ctx: &Decimal_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#decimal_value}.
 * @param ctx the parse tree
 */
fn exit_decimal_value(&mut self, _ctx: &Decimal_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#binary_value}.
 * @param ctx the parse tree
 */
fn enter_binary_value(&mut self, _ctx: &Binary_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#binary_value}.
 * @param ctx the parse tree
 */
fn exit_binary_value(&mut self, _ctx: &Binary_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#octal_value}.
 * @param ctx the parse tree
 */
fn enter_octal_value(&mut self, _ctx: &Octal_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#octal_value}.
 * @param ctx the parse tree
 */
fn exit_octal_value(&mut self, _ctx: &Octal_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hex_value}.
 * @param ctx the parse tree
 */
fn enter_hex_value(&mut self, _ctx: &Hex_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hex_value}.
 * @param ctx the parse tree
 */
fn exit_hex_value(&mut self, _ctx: &Hex_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#decimal_base}.
 * @param ctx the parse tree
 */
fn enter_decimal_base(&mut self, _ctx: &Decimal_baseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#decimal_base}.
 * @param ctx the parse tree
 */
fn exit_decimal_base(&mut self, _ctx: &Decimal_baseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#binary_base}.
 * @param ctx the parse tree
 */
fn enter_binary_base(&mut self, _ctx: &Binary_baseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#binary_base}.
 * @param ctx the parse tree
 */
fn exit_binary_base(&mut self, _ctx: &Binary_baseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#octal_base}.
 * @param ctx the parse tree
 */
fn enter_octal_base(&mut self, _ctx: &Octal_baseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#octal_base}.
 * @param ctx the parse tree
 */
fn exit_octal_base(&mut self, _ctx: &Octal_baseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hex_base}.
 * @param ctx the parse tree
 */
fn enter_hex_base(&mut self, _ctx: &Hex_baseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hex_base}.
 * @param ctx the parse tree
 */
fn exit_hex_base(&mut self, _ctx: &Hex_baseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#unbased_unsized_literal}.
 * @param ctx the parse tree
 */
fn enter_unbased_unsized_literal(&mut self, _ctx: &Unbased_unsized_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#unbased_unsized_literal}.
 * @param ctx the parse tree
 */
fn exit_unbased_unsized_literal(&mut self, _ctx: &Unbased_unsized_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#string_literal}.
 * @param ctx the parse tree
 */
fn enter_string_literal(&mut self, _ctx: &String_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#string_literal}.
 * @param ctx the parse tree
 */
fn exit_string_literal(&mut self, _ctx: &String_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#attribute_instance}.
 * @param ctx the parse tree
 */
fn enter_attribute_instance(&mut self, _ctx: &Attribute_instanceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#attribute_instance}.
 * @param ctx the parse tree
 */
fn exit_attribute_instance(&mut self, _ctx: &Attribute_instanceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#attr_spec}.
 * @param ctx the parse tree
 */
fn enter_attr_spec(&mut self, _ctx: &Attr_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#attr_spec}.
 * @param ctx the parse tree
 */
fn exit_attr_spec(&mut self, _ctx: &Attr_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#attr_name}.
 * @param ctx the parse tree
 */
fn enter_attr_name(&mut self, _ctx: &Attr_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#attr_name}.
 * @param ctx the parse tree
 */
fn exit_attr_name(&mut self, _ctx: &Attr_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#block_identifier}.
 * @param ctx the parse tree
 */
fn enter_block_identifier(&mut self, _ctx: &Block_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#block_identifier}.
 * @param ctx the parse tree
 */
fn exit_block_identifier(&mut self, _ctx: &Block_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#bin_identifier}.
 * @param ctx the parse tree
 */
fn enter_bin_identifier(&mut self, _ctx: &Bin_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#bin_identifier}.
 * @param ctx the parse tree
 */
fn exit_bin_identifier(&mut self, _ctx: &Bin_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#c_identifier}.
 * @param ctx the parse tree
 */
fn enter_c_identifier(&mut self, _ctx: &C_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#c_identifier}.
 * @param ctx the parse tree
 */
fn exit_c_identifier(&mut self, _ctx: &C_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cell_identifier}.
 * @param ctx the parse tree
 */
fn enter_cell_identifier(&mut self, _ctx: &Cell_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cell_identifier}.
 * @param ctx the parse tree
 */
fn exit_cell_identifier(&mut self, _ctx: &Cell_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#checker_identifier}.
 * @param ctx the parse tree
 */
fn enter_checker_identifier(&mut self, _ctx: &Checker_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#checker_identifier}.
 * @param ctx the parse tree
 */
fn exit_checker_identifier(&mut self, _ctx: &Checker_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_identifier}.
 * @param ctx the parse tree
 */
fn enter_class_identifier(&mut self, _ctx: &Class_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_identifier}.
 * @param ctx the parse tree
 */
fn exit_class_identifier(&mut self, _ctx: &Class_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#class_variable_identifier}.
 * @param ctx the parse tree
 */
fn enter_class_variable_identifier(&mut self, _ctx: &Class_variable_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#class_variable_identifier}.
 * @param ctx the parse tree
 */
fn exit_class_variable_identifier(&mut self, _ctx: &Class_variable_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#clocking_identifier}.
 * @param ctx the parse tree
 */
fn enter_clocking_identifier(&mut self, _ctx: &Clocking_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#clocking_identifier}.
 * @param ctx the parse tree
 */
fn exit_clocking_identifier(&mut self, _ctx: &Clocking_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#config_identifier}.
 * @param ctx the parse tree
 */
fn enter_config_identifier(&mut self, _ctx: &Config_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#config_identifier}.
 * @param ctx the parse tree
 */
fn exit_config_identifier(&mut self, _ctx: &Config_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#const_identifier}.
 * @param ctx the parse tree
 */
fn enter_const_identifier(&mut self, _ctx: &Const_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#const_identifier}.
 * @param ctx the parse tree
 */
fn exit_const_identifier(&mut self, _ctx: &Const_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#constraint_identifier}.
 * @param ctx the parse tree
 */
fn enter_constraint_identifier(&mut self, _ctx: &Constraint_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#constraint_identifier}.
 * @param ctx the parse tree
 */
fn exit_constraint_identifier(&mut self, _ctx: &Constraint_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#covergroup_identifier}.
 * @param ctx the parse tree
 */
fn enter_covergroup_identifier(&mut self, _ctx: &Covergroup_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#covergroup_identifier}.
 * @param ctx the parse tree
 */
fn exit_covergroup_identifier(&mut self, _ctx: &Covergroup_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cover_point_identifier}.
 * @param ctx the parse tree
 */
fn enter_cover_point_identifier(&mut self, _ctx: &Cover_point_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cover_point_identifier}.
 * @param ctx the parse tree
 */
fn exit_cover_point_identifier(&mut self, _ctx: &Cover_point_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#cross_identifier}.
 * @param ctx the parse tree
 */
fn enter_cross_identifier(&mut self, _ctx: &Cross_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#cross_identifier}.
 * @param ctx the parse tree
 */
fn exit_cross_identifier(&mut self, _ctx: &Cross_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#dynamic_array_variable_identifier}.
 * @param ctx the parse tree
 */
fn enter_dynamic_array_variable_identifier(&mut self, _ctx: &Dynamic_array_variable_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#dynamic_array_variable_identifier}.
 * @param ctx the parse tree
 */
fn exit_dynamic_array_variable_identifier(&mut self, _ctx: &Dynamic_array_variable_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#enum_identifier}.
 * @param ctx the parse tree
 */
fn enter_enum_identifier(&mut self, _ctx: &Enum_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#enum_identifier}.
 * @param ctx the parse tree
 */
fn exit_enum_identifier(&mut self, _ctx: &Enum_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#escaped_identifier}.
 * @param ctx the parse tree
 */
fn enter_escaped_identifier(&mut self, _ctx: &Escaped_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#escaped_identifier}.
 * @param ctx the parse tree
 */
fn exit_escaped_identifier(&mut self, _ctx: &Escaped_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#formal_port_identifier}.
 * @param ctx the parse tree
 */
fn enter_formal_port_identifier(&mut self, _ctx: &Formal_port_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#formal_port_identifier}.
 * @param ctx the parse tree
 */
fn exit_formal_port_identifier(&mut self, _ctx: &Formal_port_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#function_identifier}.
 * @param ctx the parse tree
 */
fn enter_function_identifier(&mut self, _ctx: &Function_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#function_identifier}.
 * @param ctx the parse tree
 */
fn exit_function_identifier(&mut self, _ctx: &Function_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#generate_block_identifier}.
 * @param ctx the parse tree
 */
fn enter_generate_block_identifier(&mut self, _ctx: &Generate_block_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#generate_block_identifier}.
 * @param ctx the parse tree
 */
fn exit_generate_block_identifier(&mut self, _ctx: &Generate_block_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#genvar_identifier}.
 * @param ctx the parse tree
 */
fn enter_genvar_identifier(&mut self, _ctx: &Genvar_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#genvar_identifier}.
 * @param ctx the parse tree
 */
fn exit_genvar_identifier(&mut self, _ctx: &Genvar_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hierarchical_identifier}.
 * @param ctx the parse tree
 */
fn enter_hierarchical_identifier(&mut self, _ctx: &Hierarchical_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hierarchical_identifier}.
 * @param ctx the parse tree
 */
fn exit_hierarchical_identifier(&mut self, _ctx: &Hierarchical_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#hier_ref}.
 * @param ctx the parse tree
 */
fn enter_hier_ref(&mut self, _ctx: &Hier_refContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#hier_ref}.
 * @param ctx the parse tree
 */
fn exit_hier_ref(&mut self, _ctx: &Hier_refContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#index_variable_identifier}.
 * @param ctx the parse tree
 */
fn enter_index_variable_identifier(&mut self, _ctx: &Index_variable_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#index_variable_identifier}.
 * @param ctx the parse tree
 */
fn exit_index_variable_identifier(&mut self, _ctx: &Index_variable_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_identifier}.
 * @param ctx the parse tree
 */
fn enter_interface_identifier(&mut self, _ctx: &Interface_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_identifier}.
 * @param ctx the parse tree
 */
fn exit_interface_identifier(&mut self, _ctx: &Interface_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#interface_instance_identifier}.
 * @param ctx the parse tree
 */
fn enter_interface_instance_identifier(&mut self, _ctx: &Interface_instance_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#interface_instance_identifier}.
 * @param ctx the parse tree
 */
fn exit_interface_instance_identifier(&mut self, _ctx: &Interface_instance_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#input_port_identifier}.
 * @param ctx the parse tree
 */
fn enter_input_port_identifier(&mut self, _ctx: &Input_port_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#input_port_identifier}.
 * @param ctx the parse tree
 */
fn exit_input_port_identifier(&mut self, _ctx: &Input_port_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#instance_identifier}.
 * @param ctx the parse tree
 */
fn enter_instance_identifier(&mut self, _ctx: &Instance_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#instance_identifier}.
 * @param ctx the parse tree
 */
fn exit_instance_identifier(&mut self, _ctx: &Instance_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#library_identifier}.
 * @param ctx the parse tree
 */
fn enter_library_identifier(&mut self, _ctx: &Library_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#library_identifier}.
 * @param ctx the parse tree
 */
fn exit_library_identifier(&mut self, _ctx: &Library_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#member_identifier}.
 * @param ctx the parse tree
 */
fn enter_member_identifier(&mut self, _ctx: &Member_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#member_identifier}.
 * @param ctx the parse tree
 */
fn exit_member_identifier(&mut self, _ctx: &Member_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#method_identifier}.
 * @param ctx the parse tree
 */
fn enter_method_identifier(&mut self, _ctx: &Method_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#method_identifier}.
 * @param ctx the parse tree
 */
fn exit_method_identifier(&mut self, _ctx: &Method_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#modport_identifier}.
 * @param ctx the parse tree
 */
fn enter_modport_identifier(&mut self, _ctx: &Modport_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#modport_identifier}.
 * @param ctx the parse tree
 */
fn exit_modport_identifier(&mut self, _ctx: &Modport_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#module_identifier}.
 * @param ctx the parse tree
 */
fn enter_module_identifier(&mut self, _ctx: &Module_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#module_identifier}.
 * @param ctx the parse tree
 */
fn exit_module_identifier(&mut self, _ctx: &Module_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_identifier}.
 * @param ctx the parse tree
 */
fn enter_net_identifier(&mut self, _ctx: &Net_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_identifier}.
 * @param ctx the parse tree
 */
fn exit_net_identifier(&mut self, _ctx: &Net_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#net_type_identifier}.
 * @param ctx the parse tree
 */
fn enter_net_type_identifier(&mut self, _ctx: &Net_type_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#net_type_identifier}.
 * @param ctx the parse tree
 */
fn exit_net_type_identifier(&mut self, _ctx: &Net_type_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#output_port_identifier}.
 * @param ctx the parse tree
 */
fn enter_output_port_identifier(&mut self, _ctx: &Output_port_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#output_port_identifier}.
 * @param ctx the parse tree
 */
fn exit_output_port_identifier(&mut self, _ctx: &Output_port_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_identifier}.
 * @param ctx the parse tree
 */
fn enter_package_identifier(&mut self, _ctx: &Package_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_identifier}.
 * @param ctx the parse tree
 */
fn exit_package_identifier(&mut self, _ctx: &Package_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#package_scope}.
 * @param ctx the parse tree
 */
fn enter_package_scope(&mut self, _ctx: &Package_scopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#package_scope}.
 * @param ctx the parse tree
 */
fn exit_package_scope(&mut self, _ctx: &Package_scopeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#parameter_identifier}.
 * @param ctx the parse tree
 */
fn enter_parameter_identifier(&mut self, _ctx: &Parameter_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#parameter_identifier}.
 * @param ctx the parse tree
 */
fn exit_parameter_identifier(&mut self, _ctx: &Parameter_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#port_identifier}.
 * @param ctx the parse tree
 */
fn enter_port_identifier(&mut self, _ctx: &Port_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#port_identifier}.
 * @param ctx the parse tree
 */
fn exit_port_identifier(&mut self, _ctx: &Port_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#production_identifier}.
 * @param ctx the parse tree
 */
fn enter_production_identifier(&mut self, _ctx: &Production_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#production_identifier}.
 * @param ctx the parse tree
 */
fn exit_production_identifier(&mut self, _ctx: &Production_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#program_identifier}.
 * @param ctx the parse tree
 */
fn enter_program_identifier(&mut self, _ctx: &Program_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#program_identifier}.
 * @param ctx the parse tree
 */
fn exit_program_identifier(&mut self, _ctx: &Program_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#property_identifier}.
 * @param ctx the parse tree
 */
fn enter_property_identifier(&mut self, _ctx: &Property_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#property_identifier}.
 * @param ctx the parse tree
 */
fn exit_property_identifier(&mut self, _ctx: &Property_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ps_identifier}.
 * @param ctx the parse tree
 */
fn enter_ps_identifier(&mut self, _ctx: &Ps_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ps_identifier}.
 * @param ctx the parse tree
 */
fn exit_ps_identifier(&mut self, _ctx: &Ps_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ps_or_hierarchical_array_identifier}.
 * @param ctx the parse tree
 */
fn enter_ps_or_hierarchical_array_identifier(&mut self, _ctx: &Ps_or_hierarchical_array_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ps_or_hierarchical_array_identifier}.
 * @param ctx the parse tree
 */
fn exit_ps_or_hierarchical_array_identifier(&mut self, _ctx: &Ps_or_hierarchical_array_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ps_or_hierarchical_identifier}.
 * @param ctx the parse tree
 */
fn enter_ps_or_hierarchical_identifier(&mut self, _ctx: &Ps_or_hierarchical_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ps_or_hierarchical_identifier}.
 * @param ctx the parse tree
 */
fn exit_ps_or_hierarchical_identifier(&mut self, _ctx: &Ps_or_hierarchical_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#ps_type_or_parameter_identifier}.
 * @param ctx the parse tree
 */
fn enter_ps_type_or_parameter_identifier(&mut self, _ctx: &Ps_type_or_parameter_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#ps_type_or_parameter_identifier}.
 * @param ctx the parse tree
 */
fn exit_ps_type_or_parameter_identifier(&mut self, _ctx: &Ps_type_or_parameter_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#gen_ref}.
 * @param ctx the parse tree
 */
fn enter_gen_ref(&mut self, _ctx: &Gen_refContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#gen_ref}.
 * @param ctx the parse tree
 */
fn exit_gen_ref(&mut self, _ctx: &Gen_refContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#sequence_identifier}.
 * @param ctx the parse tree
 */
fn enter_sequence_identifier(&mut self, _ctx: &Sequence_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#sequence_identifier}.
 * @param ctx the parse tree
 */
fn exit_sequence_identifier(&mut self, _ctx: &Sequence_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#signal_identifier}.
 * @param ctx the parse tree
 */
fn enter_signal_identifier(&mut self, _ctx: &Signal_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#signal_identifier}.
 * @param ctx the parse tree
 */
fn exit_signal_identifier(&mut self, _ctx: &Signal_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#simple_identifier}.
 * @param ctx the parse tree
 */
fn enter_simple_identifier(&mut self, _ctx: &Simple_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#simple_identifier}.
 * @param ctx the parse tree
 */
fn exit_simple_identifier(&mut self, _ctx: &Simple_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#specparam_identifier}.
 * @param ctx the parse tree
 */
fn enter_specparam_identifier(&mut self, _ctx: &Specparam_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#specparam_identifier}.
 * @param ctx the parse tree
 */
fn exit_specparam_identifier(&mut self, _ctx: &Specparam_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#system_tf_identifier}.
 * @param ctx the parse tree
 */
fn enter_system_tf_identifier(&mut self, _ctx: &System_tf_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#system_tf_identifier}.
 * @param ctx the parse tree
 */
fn exit_system_tf_identifier(&mut self, _ctx: &System_tf_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#task_identifier}.
 * @param ctx the parse tree
 */
fn enter_task_identifier(&mut self, _ctx: &Task_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#task_identifier}.
 * @param ctx the parse tree
 */
fn exit_task_identifier(&mut self, _ctx: &Task_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#tf_identifier}.
 * @param ctx the parse tree
 */
fn enter_tf_identifier(&mut self, _ctx: &Tf_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#tf_identifier}.
 * @param ctx the parse tree
 */
fn exit_tf_identifier(&mut self, _ctx: &Tf_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#terminal_identifier}.
 * @param ctx the parse tree
 */
fn enter_terminal_identifier(&mut self, _ctx: &Terminal_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#terminal_identifier}.
 * @param ctx the parse tree
 */
fn exit_terminal_identifier(&mut self, _ctx: &Terminal_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#topmodule_identifier}.
 * @param ctx the parse tree
 */
fn enter_topmodule_identifier(&mut self, _ctx: &Topmodule_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#topmodule_identifier}.
 * @param ctx the parse tree
 */
fn exit_topmodule_identifier(&mut self, _ctx: &Topmodule_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#type_identifier}.
 * @param ctx the parse tree
 */
fn enter_type_identifier(&mut self, _ctx: &Type_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#type_identifier}.
 * @param ctx the parse tree
 */
fn exit_type_identifier(&mut self, _ctx: &Type_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#udp_identifier}.
 * @param ctx the parse tree
 */
fn enter_udp_identifier(&mut self, _ctx: &Udp_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#udp_identifier}.
 * @param ctx the parse tree
 */
fn exit_udp_identifier(&mut self, _ctx: &Udp_identifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SystemVerilogParser#variable_identifier}.
 * @param ctx the parse tree
 */
fn enter_variable_identifier(&mut self, _ctx: &Variable_identifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SystemVerilogParser#variable_identifier}.
 * @param ctx the parse tree
 */
fn exit_variable_identifier(&mut self, _ctx: &Variable_identifierContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SystemVerilogParserListener<'input> }


