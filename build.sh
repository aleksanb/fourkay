#!/bin/bash
bindgen --rust-target nightly --distrust-clang-mangling --no-derive-debug --use-core --raw-line $'#![allow(warnings)]\n\n#[link(name="X11")]\nextern "C" {}' --ctypes-prefix 'libc' /usr/include/X11/Xlib.h > src/bindings/Xlib.rs
bindgen --rust-target nightly --distrust-clang-mangling --no-derive-debug --use-core --raw-line $'#![allow(warnings)]\n\n#[link(name="GL")]\nextern "C" {}' --ctypes-prefix 'libc' /usr/include/GL/gl.h > src/bindings/gl.rs
bindgen --rust-target nightly --distrust-clang-mangling --no-derive-debug --use-core --raw-line $'#![allow(warnings)]\n\n#[link(name="GLX")]\nextern "C" {}' --ctypes-prefix 'libc' /usr/include/GL/glx.h > src/bindings/glx.rs
bindgen --rust-target nightly --distrust-clang-mangling --no-derive-debug --use-core --raw-line $'#![allow(warnings)]\n\n#[link(name="SDL")]\nextern "C" {}' --ctypes-prefix 'libc' /usr/include/SDL/SDL.h > src/bindings/SDL.rs
