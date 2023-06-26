/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::diagnostics::Diagnostic;
use crate::slingshot_complete;
use crate::slingshot_diagnose;
use std::ffi::CString;
use std::ffi::CStr;

// This is a layer on top of the rust-bindgen generated bindings to make them more ergonomic for
// Rust

pub struct Slingshot {

}

impl Slingshot {
    pub fn diagnose(document: &str, debug: bool) -> Vec<Diagnostic> {
        let cstr = CString::new(document).expect("Failed to create document str");
        unsafe {
            let result = slingshot_diagnose(cstr.as_ptr(), debug);
            println!("Result returned: {} tokens", result.numDiagnostics);   
        }

        return Vec::new();
    }
}
