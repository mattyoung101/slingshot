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

// if necessary for debugging, this insanity appears to be the only way to print from a build script
// https://stackoverflow.com/a/75263349/5007892
// macro_rules! buildprint {
//     ($($tokens: tt)*) => {
//         println!("cargo:warning={}", format!($($tokens)*))
//     }
// }

fn main() {
    // compile slingshot with CMake first
    // TODO detect if this is a cargo release build and compile slingshot-cpp with
    // CMAKE_BUILD_TYPE=Release
    let dst = Config::new("slingshot-cpp").build();

    // reference: https://rendered-obsolete.github.io/2018/09/30/rust-ffi-ci.html
    
    // link slingshot static library, note here that we use a "fat" static library which is a
    // combination of both libslingshot (from slingshot-cpp) and libsvlang (from slang) to make
    // linking with rust possible.
    let libpath = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", libpath.display());
    println!("cargo:rustc-link-lib=static=slingshotfat");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=slingshot-cpp/src");
    println!("cargo:rerun-if-changed=slingshot-cpp/include");
    println!("cargo:rerun-if-changed=slingshot-cpp/CMakeLists.txt");

    // generate rust bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // only generate bindings for functions starting with "slingshot" to fix u128 problems
        .allowlist_function("slingshot.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings");
}
