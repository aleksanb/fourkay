#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl.rs"));
}

mod glx {
    include!(concat!(env!("OUT_DIR"), "/glx.rs"));
}

mod Xlib {
    include!(concat!(env!("OUT_DIR"), "/Xlib.rs"));
}
