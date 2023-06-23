/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use slingshot::{slingshot_get_slang_version, slingshot_free_str};
// use tower_lsp::jsonrpc::Result;
// use tower_lsp::lsp_types::*;
// use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::ffi::CStr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    unsafe {
        let raw_slang_version = slingshot_get_slang_version();
        let slang_version = CStr::from_ptr(raw_slang_version).to_str().expect("Cannot unwrap str");
        // TODO figure out why this version differs from the C++ one
        println!("Slang version, from Rust: {}", slang_version);
        slingshot_free_str(raw_slang_version);
    }
}

