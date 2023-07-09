/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use log::debug;
use serde::{Deserialize, Serialize};

pub mod completion;
pub mod diagnostics;
pub mod indexing;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub enum TokenType {
    /// A token type that does not matter to us for completion
    NotInterested,
    /// A module
    Module,
    /// A class
    Class,
    /// An enum
    Enum,
    /// A "variable": logic, wire or register
    Variable,
    /// A port in a module
    Port,
    /// A `defined macro
    Macro,
    /// A value of an enum
    EnumValue,
}

/// A SvToken contains the name of the token and its type
#[derive(Debug, PartialEq, Ord, PartialOrd, Hash, Eq, Serialize, Deserialize, Clone)]
pub struct SvToken {
    name: String,
    token_type: TokenType,
}

/// A SystemVerilog document (contains a set of modules and enums). This is used to build a very
/// simplified internal representation of an SV document for completion and indexing purposes. For
/// more information, see docs/index_design.md which covers this as well.
#[derive(Debug, PartialEq, PartialOrd, Ord, Hash, Eq, Serialize, Deserialize, Default, Clone)]
pub struct SvDocument {
    pub modules: Vec<SvModule>,
    pub enums: Vec<SvEnum>,

    cur_module: Option<SvModule>,
}

/// A SystemVerilog module which contains a public set of ports and private set of variables
#[derive(Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, Default, Clone)]
pub struct SvModule {
    /// Name of the module
    pub name: String,
    /// Public module ports
    pub ports: Vec<SvToken>,
    /// Private logic, wire, etc, tokens to this module
    pub variables: Vec<SvToken>,
}

/// SystemVerilog enum, in the form: typedef enum { ... } Enum_t;
#[derive(Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
pub struct SvEnum {
    /// List of values this enum defines
    pub enum_values: Vec<TokenType>,
}

impl SvModule {
    fn new(name: &str) -> SvModule {
        SvModule {
            name: name.to_string(),
            ports: Vec::new(),
            variables: Vec::new(),
        }
    }
}

impl SvDocument {
    fn default() -> SvDocument {
        Default::default()
    }

    /// Starts a new module in the document if one is currently not started, otherwise, ends the
    /// existing module.
    fn new_module(&mut self, name: &str) {
        if self.cur_module.is_some() {
            debug!(
                "Finishing module: {}",
                self.cur_module.as_ref().unwrap().name
            );
            // I'm also not sure if this is idiomatic Rust, but it got me past the borrow checker
            // (i.e. calling .clone())
            self.modules.push(self.cur_module.clone().unwrap());
            self.cur_module = None
        } else {
            debug!("Starting new module: {}", name);
            self.cur_module = Some(SvModule::new(name));
        }
    }

    /// Forcibly finishes the current module
    fn finish_module(&mut self) {
        assert!(self.cur_module.is_some());
        debug!(
            "Finishing current module: {}",
            self.cur_module.as_ref().unwrap().name
        );
        self.modules.push(self.cur_module.clone().unwrap());
        self.cur_module = None
    }

    /// Adds a variable to the current module
    fn add_variable(&mut self, variable: SvToken) {
        assert!(self.cur_module.is_some());
        debug!(
            "Adding variable: {} to module: {}",
            variable.name,
            self.cur_module.as_ref().unwrap().name
        );
        self.cur_module.as_mut().unwrap().variables.push(variable);
    }

    /// Adds a port to the current module
    fn add_port(&mut self, port: SvToken) {
        assert!(self.cur_module.is_some());
        debug!(
            "Adding port: {} to module: {}",
            port.name,
            self.cur_module.as_ref().unwrap().name
        );
        self.cur_module.as_mut().unwrap().ports.push(port);
    }

    /// Returns true if the given variable token has already been declared in the current module as
    /// a port.
    fn is_var_declared_as_port(&self, variable: &SvToken) -> bool {
        assert!(self.cur_module.is_some());
        return self
            .cur_module
            .as_ref()
            .unwrap()
            .ports
            .iter()
            .any(|t| t.name == variable.name && t.token_type == TokenType::Port);
    }

    /// Locates a port in a document by a partial string. This is used for auto-complete. Currently
    /// this uses a naive slow algorithm but could be made more optimal in future.
    /// Future optimisation plans would be to use trie_rs to compute a trie that maps between a
    /// name and a module.
    /// We could also do fuzzy-finding: https://github.com/lotabout/fuzzy-matcher
    pub fn locate_port(&self, port: &str) -> Option<(&SvModule, &SvToken)> {
        for module in self.modules.as_slice() {
            for p in module.ports.as_slice() {
                if p.name.contains(port) {
                    return Some((module, p));
                }
            }
        }
        None
    }

    // TODO: code for start_enum, etc
}
