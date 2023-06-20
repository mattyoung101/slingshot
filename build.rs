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
    
    // link slingshot static library
    // reference: https://rendered-obsolete.github.io/2018/09/30/rust-ffi-ci.html
    let libpath = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", libpath.display());
    println!("cargo:rustc-link-lib=static=slingshot");

    // generate rust bindings
    let bindings = bindgen::Builder::default()
        .header("slingshot.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings");
}
