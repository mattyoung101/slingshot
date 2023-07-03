/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    usize,
};

use log::{debug, warn};
use serde::{Deserialize, Serialize};
use sv_parser::{parse_sv_str, unwrap_node, Locate, RefNode};

#[derive(Debug, Eq, PartialOrd, Ord, PartialEq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    /// A token type that does not matter to us for completion
    NotInterested,
    /// A module, class or enum
    ModuleClassEnum,
    /// A logic, wire or register
    LogicWireReg,
    /// A port in a module
    Port,
    /// A `defined macro
    Macro,
    /// A value of an enum
    EnumValue,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Hash, Eq, Serialize, Deserialize)]
pub struct CompletionToken {
    token: String,
    token_type: TokenType,
}

/// Interface to a piece of software that can generate completions, e.g. sv-parser, Slang, etc
pub trait CompletionProvider {
    /// Extracts a list of CompletionTokens for the given document. Note the HashSet, as completion
    /// tokens must be unique.
    fn extract_tokens(document: &str) -> Option<HashSet<CompletionToken>>;
}

/// Diagnostics powered by Rust's sv-parser crate
pub struct SvParserCompletion {}

/// Removes a line of string from a document. If sv-parser cannot parse a document, we find the
/// source of the error, splice it, and try re-running sv-parser again. Yes, this is exactly the
/// same method that "fuckit.py" and other "error streamrollers" use, and yes it's mega stupid, but
/// we basically have no choice in the matter because nom, the library that sv-parser uses, does
/// not implement error recovery.
///
/// Note: We insert comments, not newlines, so that sv-parser's line count stays correct.
/// Note: This function uses 1-based indexing (e.g. the first line is lineno==1)
fn splice(document: &str, lineno: usize) -> String {
    let mut output = String::new();

    for (i, line) in document.lines().enumerate() {
        if i + 1 == lineno {
            // push the string with comments surrounding
            output.push_str(format!("/* {} */\n", line).as_str());
        } else {
            // just push the original string (+ a newline)
            output.push_str(format!("{}\n", line).as_str());
        }
    }

    return output;
}

/// Returns if this node is a node we are interested in for completion purposes, and if so, what
/// TokenType it corresponds to.
fn get_node_type(node: &RefNode) -> TokenType {
    match node {
        RefNode::ModuleIdentifier(_) => return TokenType::ModuleClassEnum,
        RefNode::VariableIdentifier(_) => return TokenType::LogicWireReg,
        RefNode::PortIdentifier(_) => return TokenType::Port,
        RefNode::ClassIdentifier(_) => return TokenType::ModuleClassEnum,
        RefNode::TypeIdentifier(_) => return TokenType::ModuleClassEnum,
        RefNode::EnumIdentifier(_) => return TokenType::EnumValue,
        _ => return TokenType::NotInterested,
    }
}

/// Source: https://github.com/dalance/sv-parser (README)
fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}

impl CompletionProvider for SvParserCompletion {
    fn extract_tokens(document: &str) -> Option<HashSet<CompletionToken>> {
        // The path of SystemVerilog source file (TODO get the actual path)
        let path = PathBuf::from("/tmp/test");
        // The list of defined macros (TODO provide a documented list of defined macros e.g.
        // __SLINGSHOT__)
        let predefines = HashMap::new();
        // The list of include paths
        let includes: Vec<PathBuf> = Vec::new();

        // this function does allow us to accept "incomplete" documents, however, this does not
        // appear to work very well
        // TODO if this fails, add logic to splice a max number of times until the error goes away
        let tree = match parse_sv_str(document, path, &predefines, &includes, false, false) {
            Ok(t) => t.0,
            Err(e) => {
                warn!("sv-parser rejected document: {:?}", e);
                return None;
            }
        };
        debug!("sv-parser accepted document:\n{:?}", tree);

        let mut tokens = HashSet::new();

        for node in &tree {
            let node_type = get_node_type(&node);
            if node_type != TokenType::NotInterested {
                let location = get_identifier(node);
                if location.is_some() {
                    let identifier = tree.get_str(&location).unwrap();
                    debug!(
                        "Found interesting node: {} (location: {:?}",
                        identifier, location
                    );

                    // special case: if this is a variable identifier, make sure we haven't
                    // previously recorded it as a port
                    if node_type == TokenType::LogicWireReg
                        && tokens.contains(&CompletionToken {
                            token: identifier.to_string(),
                            token_type: TokenType::Port,
                        })
                    {
                        debug!("Found node {} as Variable, but have previously recorded it as Port, skipping", identifier);
                        continue;
                    }

                    tokens.insert(CompletionToken {
                        token: identifier.to_string(),
                        token_type: node_type,
                    });
                }
            }
        }

        return Some(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splice() {
        // start with a document
        let document1 = "line1\nline2\nline3";

        // splice line 2
        let document1_noline2 = splice(document1, 2);
        assert_eq!(document1_noline2, "line1\n/* line2 */\nline3\n");

        // also splice line 3
        let document1_noline2_noline3 = splice(&document1_noline2, 3);
        assert_eq!(
            document1_noline2_noline3,
            "line1\n/* line2 */\n/* line3 */\n"
        );
    }
}
