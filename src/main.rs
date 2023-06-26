/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use slingshot::{slingshot_ergo::Slingshot, slingshot_get_slang_version, slingshot_free_str};
use std::ffi::CStr;

#[derive(Debug)]
struct Backend {

}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    unsafe {
        let raw_slang_version = slingshot_get_slang_version();
        let slang_version = CStr::from_ptr(raw_slang_version).to_str().expect("Cannot unwrap str");
        println!("Slang version, from Rust: {}", slang_version);
        slingshot_free_str(raw_slang_version);
    }

    Slingshot::diagnose("hello world", false);
}

