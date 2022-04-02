use std::{env, path::PathBuf};

use bindgen::RustTarget;

fn main() {
    //println!("cargo:rustc-cfg=println");
    //println!("rustc-link-search=/home/aleksanb/Projects/fourkay/");
    //println!("rustc-link-lib=static=4klang");

    let bindings = bindgen::Builder::default()
        .rust_target(RustTarget::Nightly)
        .trust_clang_mangling(false)
        .derive_debug(false)
        .use_core()
        .raw_line(r#"#![allow(warnings)]\n\n#[link(name="X11")]\nextern "C" {}"#)
        .ctypes_prefix("libc")
        .header("/usr/include/X11/Xlib.h")
        .generate()
        .expect("Failed building bindings for Xlib.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("Xlib.rs"))
        .expect("Coudln't write bindings.");

    let bindings = bindgen::Builder::default()
        .rust_target(RustTarget::Nightly)
        .trust_clang_mangling(false)
        .derive_debug(false)
        .use_core()
        .raw_line(r#"#![allow(warnings)]\n\n#[link(name="GL")]\nextern "C" {}"#)
        .ctypes_prefix("libc")
        .header("/usr/include/GL/gl.h")
        .generate()
        .expect("Failed building bindings for gl.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("gl.rs"))
        .expect("Coudln't write bindings.");

    let bindings = bindgen::Builder::default()
        .rust_target(RustTarget::Nightly)
        .trust_clang_mangling(false)
        .derive_debug(false)
        .use_core()
        .raw_line(r#"#![allow(warnings)]\n\n#[link(name="GLX")]\nextern "C" {}"#)
        .ctypes_prefix("libc")
        .header("/usr/include/GL/glx.h")
        .generate()
        .expect("Failed building bindings for Xlib.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("glx.rs"))
        .expect("Coudln't write bindings.");
}
