/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use lazy_static::lazy_static;
use log::{debug, warn};
use regex::Regex;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position};
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;
use tokio::process::Command;
use tower_lsp::async_trait;

#[derive(thiserror::Error, Debug)]
pub enum DiagnosticProviderError {
    #[error("Verilator may have crashed.")]
    VerilatorFailed
}

/// Interface to a piece of software that can perform diagnostics, e.g. Slang, Verilator, etc.
#[async_trait]
pub trait DiagnosticProvider {
    /// Provides a set of diagnostics for the given document.
    async fn diagnose(document: &str) -> Result<Vec<Diagnostic>, anyhow::Error>;
}

/// Wrapper around Verilator to provide diagnostics
pub struct VerilatorDiagnostics {}

// Verilator warning/error matcher regex:
// %(Warning|Error).*:([0-9]+):([0-9]+): (.*)(\n.*:.*)?
// This was developed on regex101 and also accounts for the fact that Verilator can split warnings
// across 2 lines
// Useful information is in capture groups

impl VerilatorDiagnostics {
    /// Determines the length of the line "lineno" (1-indexed) in the document "document"
    ///
    /// # Panics
    /// Panics if the lineno does not exist in the document (too large or too small)
    fn get_line_length(document: &str, lineno: u32) -> u32 {
        for (i, line) in document.lines().enumerate() {
            // Verilator uses 1-indexing for line numbers so we add 1 here
            if i + 1 == TryInto::<usize>::try_into(lineno).unwrap() {
                return line.len().try_into().unwrap();
            }
        }
        panic!(
            "Lineno {} does not exist in document:\n{}",
            lineno, document
        );
    }
}

#[async_trait]
impl DiagnosticProvider for VerilatorDiagnostics {
    async fn diagnose(document: &str) -> Result<Vec<Diagnostic>, anyhow::Error> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        // write document to a temp file (verilator doesn't allow stdin as an input)
        let mut tmpfile = NamedTempFile::new()?;

        // if we can create a file in /tmp we can probably write to it, just silently fail if we
        // can't for some reason -> Verilator will mald later anyway which we can detect
        // TODO note since we get a URI from LSP, we may not actually need to write out to a temp
        // file
        let _ = tmpfile.write(document.as_bytes())?;

        let tmpfile_path = tmpfile.path().to_str().unwrap();
        debug!("Created Verilator temp file: {}", tmpfile_path);

        // invoke verilator
        // note that we need to disable some warnings like DECLFILENAME because we use the temp
        // file
        let output = Command::new("verilator")
            .args(["--lint-only", "-Wall", "-Wno-DECLFILENAME", tmpfile_path])
            .output()
            .await?;

        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        let stderr = std::str::from_utf8(&output.stderr).unwrap();

        // debug!("{}", stdout);
        // debug!("{}", stderr);

        if !output.status.success() {
            debug!(
                "Verilator exited with error status: {}",
                output.status.code().unwrap()
            );

            if !(stderr.contains("Error") || stderr.contains("Warning")) {
                warn!(
                    "Verilator exited with error status, but output does not appear to contain \
                        any warning/error messages. Maybe Verilator crashed?"
                );
                warn!("Verilator said:\nstdout:\n{}\nstderr:\n{}", stdout, stderr);
                return Err(DiagnosticProviderError::VerilatorFailed.into());
            }
        }

        // parse verilator output with regex
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"%(Warning|Error).*:([0-9]+):([0-9]+): (.*)(\n.*:.*)?").unwrap();
        }

        for capture in RE.captures_iter(stderr) {
            let message_type = &capture[1];
            // if we fail to parse the line/pos number for some reason, just return 0
            // we could ignore this diagnostic entirely but may as well return it, realistically
            // this shouldn't happen anyway and the user will see **something**
            // if people start malding on the bug tracker or whatever then yeah skip the diagnostic
            let line = &capture[2].parse::<u32>().unwrap_or(0);
            let pos = &capture[3].parse::<u32>().unwrap_or(0);
            let msg1 = capture[4].to_string();

            //debug!("line: {}, pos: {}, msg1: {}", line, pos, msg1);

            // For now, we will just report to the user that the error is the entire line
            let end_pos = VerilatorDiagnostics::get_line_length(document, *line);
            //debug!("end_pos: {}", end_pos);
            
            let mut diagnostic = Diagnostic::default();
            diagnostic.message = msg1;
            diagnostic.source = Some("verilator".to_string());
            diagnostic.severity = if message_type == "Error" {
                Some(DiagnosticSeverity::ERROR)
            } else {
                Some(DiagnosticSeverity::WARNING)
            };
            // note that LSP uses 0-indexed ranges but Verilator uses 1-indexed
            let range = tower_lsp::lsp_types::Range {
                start: Position {
                    line: line - 1,
                    character: pos - 1
                },
                end: Position {
                    line: line - 1,
                    character: end_pos - 1
                }
            };
            diagnostic.range = range;
            diagnostics.push(diagnostic);
        }

        // Determine ranges for errors based on Verilator output. We look for lines that contain a
        // |, ^ and ~, which are the error squiggles, and count these to determine where the error
        // occurred. Assumes that the error squiggle lines are inserted in the same order as the
        // regex captures, which should be true unless we choose to manually ignore any
        // warnings/errors.
        // for line in stderr.lines() {
        //     if line.contains("|") && line.contains("^") && line.contains("~") {
        //         debug!("Found error squiggle line: {}", line);
        //     }
        // }
        //  TODO count verilator squiggles to determine start and end position on line

        // TODO: consider returning LSP diagnostic type directly, or making a function to convert?

        Ok(diagnostics)
    }
}
