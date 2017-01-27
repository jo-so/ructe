extern crate ructe;

use ructe::{compile_templates, compile_static_css};
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    compile_static_css(&in_dir.join("static"), &out_dir).unwrap();
    compile_templates(&in_dir.join("templates"), &out_dir).unwrap();
}
