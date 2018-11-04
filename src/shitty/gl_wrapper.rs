use core;
use crate::bindings::{gl, glx};
use libc;

pub static mut GL_GET_STRING_PTR: *const libc::c_char = core::ptr::null();
pub unsafe fn glGetString(name: gl::GLenum) -> *const gl::GLubyte {
    let function: unsafe extern "C" fn(name: gl::GLenum)
        -> *const gl::GLubyte = core::mem::transmute(GL_GET_STRING_PTR);
    function(name)
}

//pub static mut GL_GEN_VERTEX_ARRAYS: *const libc::c_char = core::ptr::null();
//pub unsafe fn glGenVertexArraysProc(num: libc::c_int) {
//    //glx::PFNGLGENVERTEXARRAYSPROC
//}

pub fn load_extensions() {
    unsafe {
        GL_GET_STRING_PTR = get_proc_address("glGetString\0");
        //GL_GEN_VERTEX_ARRAYS = get_proc_address("glGenVertexArrays\0");
    }
}

unsafe fn get_proc_address(name: &str) -> *const libc::c_char {
    let symbol = name.as_bytes();
    let address = symbol.as_ptr();
    glx::glXGetProcAddress(address as *const _).unwrap() as * const _
}
