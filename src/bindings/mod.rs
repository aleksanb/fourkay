#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl.rs"));
}

pub mod glx {
    include!(concat!(env!("OUT_DIR"), "/glx.rs"));
}

pub mod Xlib {
    include!(concat!(env!("OUT_DIR"), "/Xlib.rs"));
}

pub mod math {
    include!(concat!(env!("OUT_DIR"), "/math.rs"));
}

pub mod alsa {
    include!(concat!(env!("OUT_DIR"), "/alsa.rs"));
}

pub mod Xlib_constants;
