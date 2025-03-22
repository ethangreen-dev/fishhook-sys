use std::env;
use std::path::PathBuf;

use bindgen::Builder;

fn main() {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    println!("cargo:rustc-link-search={}/fishhook", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-lib=fishhook");

    Builder::default()
        .header("fishhook.h")
        .clang_arg("-DFISHHOOK_EXPORT")
        .formatter(bindgen::Formatter::Prettyplease)
        .generate()
        .unwrap()
        .write_to_file(project_dir.join("lib.rs"))
        .unwrap();
}
