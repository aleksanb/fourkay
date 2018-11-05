use super::println;
use core;
use crate::bindings::{gl, glx};
use libc;

unsafe fn get_proc_address(name: &str) -> *const libc::c_char {
    let symbol = name.as_bytes();
    let address = symbol.as_ptr();
    glx::glXGetProcAddress(address as *const _).unwrap() as *const _
}

macro_rules! gl_function {
($(pub unsafe fn $gl_symbol:ident( $ ( $param_name:ident: $ param_type:ty), * ) -> $ ret_type:ty),*) => {
    $(
        mod $gl_symbol {
            use crate::bindings::{gl};

            pub static mut pointer: *const libc::c_char = core::ptr::null();


            pub unsafe fn function($($param_name: $param_type),*) -> $ret_type {
                let function: unsafe extern "C" fn($($param_name: $param_type),*) -> $ret_type = core::mem::transmute(pointer);
                function($($param_name),*)
            }
        }

        pub use self::$gl_symbol::function as $gl_symbol;
    )*

    pub fn load_extensions() {
        unsafe {
            $(
                $gl_symbol::pointer = get_proc_address(concat!(stringify!($gl_symbol),"\0"));
            )*
        }
    }
}
}

gl_function!{
    pub unsafe fn glGetString(name: gl::GLenum) -> *const gl::GLubyte,
    pub unsafe fn glGenVertexArrays(n: gl::GLsizei, arrays: *mut gl::GLuint) -> *const gl::GLubyte
}
