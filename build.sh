#!/bin/bash
bindgen --distrust-clang-mangling --no-copy \* --no-derive-debug --use-core --raw-line '#[link(name="X11")]' --ctypes-prefix 'libc' Xlib.h > Xlib.rs
