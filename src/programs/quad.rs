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
        //let vertex_shader =
            //gl_utils::create_shader(&gl_utils::ShaderType::VertexShader(vertex_shader))?;
        //let program = gl_utils::create_program(fragment_shader, vertex_shader)?;
        //gl_wrapper::glUseProgram(program);

        //const NUM_VERTEX_ARRAYS: gl::GLsizei = 1;
        //let vertex_arrays: &mut [gl::GLuint] = &mut [0; NUM_VERTEX_ARRAYS as usize];
        //gl_wrapper::glGenVertexArrays(NUM_VERTEX_ARRAYS, vertex_arrays.as_ptr() as *mut _);
        //let vao = vertex_arrays[0];
        //println!("vao = %d\n\0", vao);
        //gl_wrapper::glBindVertexArray(vao);

        //const NUM_BUFFERS: gl::GLsizei = 1;
        //let buffers: &mut [gl::GLuint] = &mut [0; NUM_BUFFERS as usize];
        //gl_wrapper::glGenBuffers(NUM_BUFFERS, buffers.as_ptr() as *mut _);
        //let vbo = buffers[0];

        //let quad: &[f32] = &[
            //-1.0, 1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0,
            //-1.0, 0.0,
        //];
        //gl_wrapper::glBindBuffer(gl::GL_ARRAY_BUFFER, vbo);
        //gl_wrapper::glBufferData(
            //gl::GL_ARRAY_BUFFER,
            //(quad.len() * mem::size_of::<f32>()) as gl::GLsizeiptr,
            //quad.as_ptr() as *const _,
            //gl::GL_STATIC_DRAW,
        //);
        //gl_wrapper::glEnableVertexAttribArray(0);
        //gl_wrapper::glVertexAttribPointer(
            //0 as _,
            //3,
            //gl::GL_FLOAT,
            //gl::GL_FALSE as gl::GLboolean,
            //0,
            //core::ptr::null(),
        //);

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

    fn update(&mut self, _frame: u64) {}

    fn render(&self, frame: u64) {
        gl_wrapper::glUseProgram(self.program);

        let uniform_frame =
            gl_wrapper::glGetUniformLocation(self.program, "f\0".as_ptr() as *const _);
        gl_wrapper::glUniform1f(uniform_frame, frame as f32);

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
