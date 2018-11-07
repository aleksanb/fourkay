use crate::bindings::glx;
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
            use crate::bindings::{gl::{self, GLchar, GLenum, GLsizei, GLubyte, GLuint, GLint, GLsizeiptr, GLboolean}};
            use crate::shitty::println::*;

            pub static mut raw_pointer: *const libc::c_char = core::ptr::null();


            pub fn function($($param_name: $param_type),*) -> $ret_type {
                unsafe {
                    let function: unsafe extern "C" fn($($param_name: $param_type),*) -> $ret_type = core::mem::transmute(raw_pointer);
                    let result = function($($param_name),*);
                    let error = gl::glGetError();
                    println!("Error? %d\n\0", error);
                    assert_eq!(error, gl::GL_NO_ERROR);
                    result
                }
            }
        }

        pub use self::$gl_symbol::function as $gl_symbol;
    )*

    pub fn load_extensions() {
        unsafe {
            $(
                $gl_symbol::raw_pointer = get_proc_address(concat!(stringify!($gl_symbol),"\0"));
            )*
        }
    }
}
}

gl_function! {
    pub fn glGetString(name: GLenum) -> *const GLubyte,
    pub fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    ) -> (),
    pub fn glCreateShader(type_: GLenum) -> GLuint,
    pub fn glCompileShader(shader: GLuint) -> GLuint,
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> (),
    pub fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> (),
    pub fn glBindVertexArray(array: GLuint) -> (),
    pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) -> (),
    pub fn glBindBuffer(target: GLenum, buffer: GLuint) -> (),
    pub fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const libc::c_void,
        usage: GLenum
    ) -> (),
    pub fn glEnableVertexAttribArray(index: GLuint) -> (),
    pub fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const libc::c_void
    ) -> ()
}
