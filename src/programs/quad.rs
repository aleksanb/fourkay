use crate::bindings::{gl, glx, Xlib, Xlib_constants};
use crate::shitty::{gl_utils, gl_wrapper, println::*};

use crate::programs::Program;
use core::mem;
use core::panic::PanicInfo;
use core::ptr;

pub struct Quad {
    program: gl::GLuint,
    width: i32,
    height: i32,
}

impl Quad {
    pub fn new(fragment_shader: &'static str, vertex_shader: &'static str) -> Result<Self, ()> {
        let program =
            gl_utils::create_shader(&gl_utils::ShaderType::FragmentShader(fragment_shader))?;

        Ok(Self {
            program,
            width: 1920,
            height: 1080,
        })
    }
}

impl Program for Quad {
    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    fn update(&mut self, _time: f32) {}

    fn render(&self, time: f32) {
        gl_wrapper::glUseProgram(self.program);

        let uniform_frame =
            gl_wrapper::glGetUniformLocation(self.program, "f\0".as_ptr() as *const _);
        gl_wrapper::glUniform1f(uniform_frame, time as f32);

        let uniform_resolution =
            gl_wrapper::glGetUniformLocation(self.program, "r\0".as_ptr() as *const _);
        gl_wrapper::glUniform2f(
            uniform_resolution,
            self.width as gl::GLfloat,
            self.height as gl::GLfloat,
        );

        unsafe {
            gl_wrapper::glRects(-1, -1, 1, 1);
        }
    }
}
