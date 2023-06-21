/*
 * Copyright (c) 2023 Matt Young.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use cmake;
use std::env;
use std::path::PathBuf;
use cmake::Config;

// https://stackoverflow.com/a/75263349/5007892
// macro_rules! buildprint {
//     ($($tokens: tt)*) => {
//         println!("cargo:warning={}", format!($($tokens)*))
//     }
// }

fn main() {
    // compile slingshot with CMake first
    let dst = Config::new("slingshot-cpp").build();

    // reference: https://rendered-obsolete.github.io/2018/09/30/rust-ffi-ci.html
    
    // link slingshot static library
    let libpath = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", libpath.display());
    println!("cargo:rustc-link-lib=static=slingshot");
    // TODO rerun only if slingshot.cpp/slingshot.h/wrapper.hpp changed

    // generate rust bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings");
}
