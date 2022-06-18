use core::panic;
use std::collections::HashSet;
use std::env;

use std::path::PathBuf;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

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
        Lib {
            link_name: "m".to_string(),
            header_path: "/usr/include/math.h".to_string(),
            out_name: "math.rs".to_string(),
        },
        //Lib {
        //link_name: "pcm".to_string(),
        //header_path: "/usr/include/alsa/asoundlib.h".to_string(),
        //out_name: "alsa.rs".to_string(),
        //},
    ];

    for lib in &libraries {
        let mut bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::Nightly)
            .trust_clang_mangling(false)
            .derive_copy(false)
            .derive_debug(false)
            .use_core()
            .raw_line(format!(r#"#[link(name="{}")] extern{{}}"#, lib.link_name))
            .ctypes_prefix("libc")
            .header(lib.header_path.clone());

        if lib.out_name == "math.rs" {
            let ignored_macros = IgnoreMacros(
                vec![
                    "FP_INFINITE".into(),
                    "FP_NAN".into(),
                    "FP_NORMAL".into(),
                    "FP_SUBNORMAL".into(),
                    "FP_ZERO".into(),
                    "IPPORT_RESERVED".into(),
                ]
                .into_iter()
                .collect(),
            );

            bindings = bindings
                .parse_callbacks(Box::new(ignored_macros))
                .rustfmt_bindings(true);
        }

        let bindings = bindings.generate().expect(&format!(
            "Couldn't generate bindings for {}",
            lib.header_path
        ));

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join(&lib.out_name))
            .expect("Couldn't write bindings!");
    }

    install_alsa_sys()
}

// MIT License: https://github.com/diwic/alsa-sys/blob/master/build.rs
fn install_alsa_sys() {
    match pkg_config::Config::new().probe("alsa") {
        Err(pkg_config::Error::Failure { command, output }) => panic!(
            "Pkg-config failed - usually this is because alsa development headers are not installed.\n\n\
            For Fedora users:\n# dnf install alsa-lib-devel\n\n\
            For Debian/Ubuntu users:\n# apt-get install libasound2-dev\n\n\
            pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
        Err(e) => panic!("{}", e),
        Ok(_alsa_library) => {
            generate_bindings(&_alsa_library);
        } 
    };
}

fn generate_bindings(alsa_library: &pkg_config::Library) {
    let mut codegen_config = bindgen::CodegenConfig::empty();
    codegen_config.insert(bindgen::CodegenConfig::FUNCTIONS);
    codegen_config.insert(bindgen::CodegenConfig::TYPES);

    let builder = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Nightly)
        .derive_copy(false)
        .derive_debug(false)
        .use_core()
        //.raw_line(r#"#[link(name="alsa")] extern {}"#)
        .ctypes_prefix("libc")
        .header("/usr/include/alsa/asoundlib.h")
        //.header("alsa-wrapper.h")
        .size_t_is_usize(true)
        //.whitelist_recursively(false)
        .prepend_enum_name(false)
        .layout_tests(false)
        //.whitelist_function("snd_.*")
        //.whitelist_type("_?snd_.*")
        //.whitelist_type(".*va_list.*")
        //.with_codegen_config(codegen_config)
        //.clang_args(clang_include_args)
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks));
        ;
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("alsa.rs"))
        .expect("Couldn't write bindings");
}
