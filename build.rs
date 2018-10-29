//! Generate libfirm bindings
use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

const LIBFIRM_DIR: &str = "libfirm";

const INCLUDE_PATHS: &[&str] = &[
    "libfirm/include/libfirm",
    "libfirm/build/gen/include/libfirm"
];

macro_rules! tell_cargo {
    ($opt: expr, $val: expr) => {
        println!("cargo:{}={}", $opt, $val)
    };
}

fn main() {

    let exit_code = Command::new("make")
        .current_dir(LIBFIRM_DIR)
        .status()
        .expect("Failed to run make")
        .code()
        .unwrap_or(1);

    if exit_code != 0 {
        eprintln!("Failed to build libfirm");
        exit(exit_code);
    }

    INCLUDE_PATHS.iter().for_each(|p| tell_cargo!("include", p));
    tell_cargo!("rustc-link-search", "libfirm/build/debug");
    tell_cargo!("rustc-link-lib", "firm");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(INCLUDE_PATHS.iter().map(|p| format!("-I{}", p)))
        .generate()
        .expect("Failed to generate bindings");

    let out_path: PathBuf = env::var("OUT_DIR")
        .expect("Environment variable OUT_DIR is unset")
        .into();

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
