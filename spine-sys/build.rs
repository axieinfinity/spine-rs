extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let dist = cmake::build("spine-runtimes/spine-c").join("dist/lib");

    println!("cargo:rustc-link-search=native={}", dist.display());
    println!("cargo:rustc-link-lib=static=spine-c");

    let bindings = bindgen::Builder::default()
        .header("spine-runtimes/spine-c/spine-c/include/spine/spine.h")
        .clang_arg("-I./spine-runtimes/spine-c/spine-c/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
