/*
use crate::bindings::{gl, glx, Xlib, Xlib_constants};
use crate::shitty::{gl_utils, gl_wrapper, println::*};

use crate::programs::Program;
use core::mem;
use core::panic::PanicInfo;
use core::ptr;

static VERTEX_SHADER: &'static str = concat!(include_str!("../shaders/quad-vertex.glsl"), "\0");
static FRAGMENT_SHADER: &'static str =
    concat!(include_str!("../shaders/raymarcher-fragment.glsl"), "\0");

pub struct Raymarcher {
    program: gl::GLuint,
    width: i32,
    height: i32,
}

impl Raymarcher {
    pub fn new() -> Result<Self, ()> {
        let program =
            gl_utils::create_shader(&gl_utils::ShaderType::FragmentShader(FRAGMENT_SHADER))?;

        Ok(Self {
            program,
            width: 0,
            height: 0,
        })
    }
}

impl Program for Raymarcher {
    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    fn update(&mut self, _time: f32) {}

    fn render(&self, time: f32) {
        gl_wrapper::glUseProgram(self.program);

        let uniform_frame =
            gl_wrapper::glGetUniformLocation(self.program, "frame\0".as_ptr() as *const _);
        gl_wrapper::glUniform1f(uniform_frame, time);

        let uniform_eye =
            gl_wrapper::glGetUniformLocation(self.program, "eye\0".as_ptr() as *const _);
        gl_wrapper::glUniform3f(uniform_eye, 0.0, 0.0, 10.0);

        let uniform_forward =
            gl_wrapper::glGetUniformLocation(self.program, "forward\0".as_ptr() as *const _);
        gl_wrapper::glUniform3f(uniform_forward, 0.0, 0.0, -1.0);

        let uniform_resolution =
            gl_wrapper::glGetUniformLocation(self.program, "resolution\0".as_ptr() as *const _);
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

 */
