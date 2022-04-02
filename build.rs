use std::env;
use std::path::PathBuf;

struct Lib {
    pub link_name: String,
    pub header_path: String,
    pub out_name: String,
}

fn main() {
    let libraries = [
        Lib {
            link_name: "X11".to_string(),
            header_path: "/usr/include/X11/Xlib.h".to_string(),
            out_name: "Xlib.rs".to_string(),
        },
        Lib {
            link_name: "GL".to_string(),
            header_path: "/usr/include/GL/gl.h".to_string(),
            out_name: "gl.rs".to_string(),
        },
        Lib {
            link_name: "GLX".to_string(),
            header_path: "/usr/include/GL/glx.h".to_string(),
            out_name: "glx.rs".to_string(),
        },
    ];

    for lib in &libraries {
        let bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::Nightly)
            .trust_clang_mangling(false)
            .derive_copy(false)
            .derive_debug(false)
            .use_core()
            .raw_line(format!(r#"#[link(name="{}")] extern{{}}"#, lib.link_name))
            .ctypes_prefix("libc")
            .header(lib.header_path.clone())
            .generate()
            .expect(&format!(
                "Couldn't generate bindings for {}",
                lib.header_path
            ));

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join(&lib.out_name))
            .expect("Couldn't write bindings!");
    }
}
