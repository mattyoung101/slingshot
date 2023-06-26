/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// A diagnostic message from a diagnostic engine
pub struct Diagnostic {
    /// Diagnostic message
    message: String,
    /// Which line this occurs on
    line: usize,
    /// Position in line this occurs on
    offset: usize
}

/// Interface to a piece of software that can perform diagnostics, e.g. Slang, Verilator, etc.
pub trait DiagnosticProvider {

}
