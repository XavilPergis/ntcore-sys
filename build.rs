extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell rustc to link to libntcore.
    println!("cargo:rustc-link-lib=ntcore");
    println!("cargo:rustc-link-search={}/lib", env::current_dir().unwrap().display());

    let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindgen::Builder::default()
        .header("ntcore_c.h")
        .whitelist_type("NT_.*")
        .whitelist_function("NT_.*")
        .whitelist_var("NT_.*") // Whitelist all the NT_* items
        .generate()
        .expect("Failed to generate binding for ntcore.")
        .write_to_file(path).expect("Failed writing ntcore bindings to output file.");
}
