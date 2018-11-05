use crate::bindings::{
    glx,
};
use libc;

unsafe fn get_proc_address(name: &str) -> *const libc::c_char {
    let symbol = name.as_bytes();
    let address = symbol.as_ptr();
    glx::glXGetProcAddress(address as *const _).unwrap() as *const _
}

macro_rules! gl_function {
($(pub fn $gl_symbol:ident( $ ( $param_name:ident: $ param_type:ty), * ) -> $ ret_type:ty),*) => {
    $(
        mod $gl_symbol {
            use crate::bindings::{gl::{GLchar, GLenum, GLsizei, GLubyte, GLuint, GLint}};

            pub static mut pointer: *const libc::c_char = core::ptr::null();


            pub fn function($($param_name: $param_type),*) -> $ret_type {
                unsafe {
                    let function: unsafe extern "C" fn($($param_name: $param_type),*) -> $ret_type = core::mem::transmute(pointer);
                    function($($param_name),*)
                }
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
    pub fn glGetString(name: GLenum) -> *const GLubyte,
    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> *const GLubyte,
    pub fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    ) -> (),
    pub fn glCreateShader(type_: GLenum) -> GLuint,
    pub fn glCompileShader(shader: GLuint) -> GLuint
}
