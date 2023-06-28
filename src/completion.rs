/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{path::PathBuf, collections::HashMap};

use log::debug;
use sv_parser::parse_sv_str;

#[derive(Debug)]
pub enum TokenType {
    /// A module, class or enum
    ModuleClassEnum,
    /// A logic, wire or register
    LogicWireReg,
    /// A port in a module
    Port,
    /// A `defined macro
    Macro,
    /// A value of an enum
    EnumValue
}

#[derive(Debug)]
pub struct CompletionToken {
    token: String,
    token_type: TokenType
}

/// Interface to a piece of software that can generate completions, e.g. sv-parser, Slang, etc
pub trait CompletionProvider {
    /// Extracts a list of CompletionTokens for the given document
    fn extract_tokens(document: &str) -> Vec<CompletionToken>;
}

/// Diagnostics powered by Rust's sv-parser crate
pub struct SvParserCompletion {

}

impl CompletionProvider for SvParserCompletion {
    fn extract_tokens(document: &str) -> Vec<CompletionToken> {
        // The path of SystemVerilog source file
        let path = PathBuf::from("/tmp/test");
        // The list of defined macros
        let predefines = HashMap::new();
        // The list of include paths
        let includes: Vec<PathBuf> = Vec::new();

        let (tree, _) = parse_sv_str(document, path, &predefines, &includes, false, true).unwrap();
        debug!("sv-parser says:\n{:?}", tree);

        return Vec::new();
    }
}
