#!/bin/bash
bindgen --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")]' --ctypes-prefix 'libc' /usr/include/X11/Xlib.h > src/ffi/Xlib.rs
bindgen --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")]' --ctypes-prefix 'libc' /usr/include/GL/gl.h > src/ffi/gl.rs
bindgen --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")]' --ctypes-prefix 'libc' /usr/include/GL/glx.h > src/ffi/glx.rs
