#!/bin/bash
bindgen --rust-target nightly --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")] extern{}' --ctypes-prefix 'libc' /usr/include/X11/Xlib.h > src/bindings/Xlib.rs
bindgen --rust-target nightly --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")] extern{}' --ctypes-prefix 'libc' /usr/include/GL/gl.h > src/bindings/gl.rs
bindgen --rust-target nightly --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")] extern{}' --ctypes-prefix 'libc' /usr/include/GL/glx.h > src/bindings/glx.rs
