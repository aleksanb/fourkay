#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bindings::glx;
use libc;

unsafe fn get_proc_address(name: &str) -> *const libc::c_char {
    let symbol = name.as_bytes();
    let address = symbol.as_ptr();
    glx::glXGetProcAddress(address as *const _).unwrap() as *const _
}

macro_rules! gl_function {
($($(#[$attributes:meta])? fn $gl_symbol:ident( $ ( $param_name:ident: $ param_type:ty), * ) -> $ ret_type:ty),*) => {
    $(
        $(#[$attributes])?
        mod $gl_symbol {
            #[allow(unused_imports)]
            use crate::bindings::{gl::{self, GLchar, GLshort, GLenum, GLsizei, GLubyte, GLuint, GLint, GLsizeiptr, GLboolean, GLfloat}};

            pub(crate) static mut RAW_POINTER: *const libc::c_char = core::ptr::null();

            pub(crate) fn function($($param_name: $param_type),*) -> $ret_type {
                unsafe {
                    let function: unsafe extern "C" fn($($param_name: $param_type),*) -> $ret_type = core::mem::transmute(RAW_POINTER);
                    function($($param_name),*)
                }
            }
        }

        $(#[$attributes])?
        pub(crate) use self::$gl_symbol::function as $gl_symbol;
    )*

    pub(crate) fn load_extensions() {
        $(
            $(#[$attributes])?
            unsafe {
                $gl_symbol::RAW_POINTER = get_proc_address(concat!(stringify!($gl_symbol),"\0"));
            }
        )*
    }
}
}

gl_function! {
    fn glCreateShaderProgramv(type_: GLenum, count: GLint, strings: *const *const GLchar) -> GLuint,
    fn glUseProgram(program: GLuint) -> (),
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint,
    fn glUniform1f(location: GLint, v0: GLfloat) -> (),
    // fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> (),
    fn glRects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) -> (),

    #[cfg(feature = "error-handling")]
    fn glGetString(name: GLenum) -> *const GLubyte,
    #[cfg(feature = "error-handling")]
    fn glCreateShader(type_: GLenum) -> GLuint,
    #[cfg(feature = "error-handling")]
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    ) -> (),
    #[cfg(feature = "error-handling")]
    fn glCompileShader(shader: GLuint) -> GLuint,
    #[cfg(feature = "error-handling")]
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> (),
    #[cfg(feature = "error-handling")]
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    #[cfg(feature = "error-handling")]
    fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glBindVertexArray(array: GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glBindBuffer(target: GLenum, buffer: GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const libc::c_void,
        usage: GLenum
    ) -> (),
    #[cfg(feature = "error-handling")]
    fn glEnableVertexAttribArray(index: GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const libc::c_void
    ) -> (),
    #[cfg(feature = "error-handling")]
    fn glCreateProgram() -> GLuint,
    #[cfg(feature = "error-handling")]
    fn glAttachShader(program: GLuint, shader: GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glLinkProgram(program: GLuint) -> (),
    #[cfg(feature = "error-handling")]
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> (),
    #[cfg(feature = "error-handling")]
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    #[cfg(feature = "error-handling")]
    fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> (),
    #[cfg(feature = "error-handling")]
    fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> ()
}
