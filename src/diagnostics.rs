/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use lazy_static::lazy_static;
use log::{debug, error, warn};
use regex::Regex;
use std::{io::Write, process::Command};
use tempfile::NamedTempFile;

/// A diagnostic message from a diagnostic engine
#[derive(Debug)]
pub struct Diagnostic {
    /// Diagnostic message
    message: String,
    /// Which line this occurs on
    line: usize,
    /// Position in line this occurs on
    offset: usize,
}

/// Interface to a piece of software that can perform diagnostics, e.g. Slang, Verilator, etc.
pub trait DiagnosticProvider {
    /// Provides a set of diagnostics for the given document.
    fn diagnose(document: &str) -> Option<Vec<Diagnostic>>;
}

/// Wrapper around Verilator to provide diagnostics
pub struct VerilatorDiagnostics {}

// Verilator warning/error matcher regex:
// %(Warning|Error).*:([0-9]+):([0-9]+): (.*)(\n.*:.*)?
// This was developed on regex101 and also accounts for the fact that Verilator can split warnings
// across 2 lines
// Useful information is in capture groups

impl DiagnosticProvider for VerilatorDiagnostics {
    fn diagnose(document: &str) -> Option<Vec<Diagnostic>> {
        debug!("Running VerilatorDiagnostics for document:\n{}", document);

        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        // write document to a temp file (verilator doesn't allow stdin as an input)
        let mut tmpfile = match NamedTempFile::new() {
            Ok(file) => file,
            Err(err) => {
                error!("Error creating temp file: {}", err);
                return None;
            }
        };
        // if we can create a file in /tmp we can probably write to it, just silently fail if we
        // can't for some reason -> Verilator will mald later anyway which we can detect
        let _ = tmpfile.write(document.as_bytes());

        let tmpfile_path = tmpfile.path().to_str().unwrap();
        debug!("Created Verilator temp file: {}", tmpfile_path);

        // invoke verilator
        // note that we need to disable some warnings like DECLFILENAME because we use the temp
        // file
        let output = match Command::new("verilator")
            .args(["--lint-only", "-Wall", "-Wno-DECLFILENAME", tmpfile_path])
            .output()
        {
            Ok(o) => o,
            Err(err) => {
                error!("Failed to invoke Verilator: {}", err);
                return None;
            }
        };

        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        let stderr = std::str::from_utf8(&output.stderr).unwrap();

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
                return None;
            }
        }

        // parse verilator output with regex
        // TODO move this to the verilator struct somehow
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"%(Warning|Error).*:([0-9]+):([0-9]+): (.*)(\n.*:.*)?").unwrap();
        }

        for capture in RE.captures_iter(stderr) {
            //let message_type = &capture[1];
            // if we fail to parse the line/pos number for some reason, just return 0
            // we could ignore this diagnostic entirely but may as well return it, realistically
            // this shouldn't happen anyway and the user will see **something**
            // if people start malding on the bug tracker or whatever then yeah skip the diagnostic
            let line = &capture[2].parse::<usize>().unwrap_or(0);
            let pos = &capture[3].parse::<usize>().unwrap_or(0);
            let msg1 = capture[4].to_string();

            diagnostics.push(Diagnostic {
                message: msg1.to_string(),
                line: *line,
                offset: *pos,
            });
        }

        return Some(diagnostics);
    }
}
