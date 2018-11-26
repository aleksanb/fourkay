use crate::bindings::{gl, glx, Xlib, Xlib_constants};
use crate::shitty::{gl_utils, gl_wrapper, println::*};

use crate::programs::Program;
use core::mem;
use core::panic::PanicInfo;
use core::ptr;

static VERTEX_SHADER: &'static str = concat!(include_str!("../shaders/quad-vertex.glsl"), "\0");
static FRAGMENT_SHADER: &'static str = concat!(include_str!("../shaders/quad-fragment.glsl"), "\0");

pub struct Quad {
    program: gl::GLuint,
}

impl Program for Quad {
    fn new() -> Result<Quad, ()> {
        let fragment_shader =
            gl_utils::create_shader(&gl_utils::ShaderType::FragmentShader(FRAGMENT_SHADER))?;
        let vertex_shader =
            gl_utils::create_shader(&gl_utils::ShaderType::VertexShader(VERTEX_SHADER))?;
        let program = gl_utils::create_program(fragment_shader, vertex_shader)?;
        gl_wrapper::glUseProgram(program);

        const num_vertex_arrays: gl::GLsizei = 1;
        let vertex_arrays: &mut [gl::GLuint] = &mut [0; num_vertex_arrays as usize];
        gl_wrapper::glGenVertexArrays(num_vertex_arrays, vertex_arrays.as_ptr() as *mut _);
        let vao = vertex_arrays[0];
        println!("vao = %d\n\0", vao);
        gl_wrapper::glBindVertexArray(vao);

        const num_buffers: gl::GLsizei = 1;
        let buffers: &mut [gl::GLuint] = &mut [0; num_buffers as usize];
        gl_wrapper::glGenBuffers(num_buffers, buffers.as_ptr() as *mut _);
        let vbo = buffers[0];

        let quad: &[f32] = &[
            -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0,
            -1.0, 0.0,
        ];
        gl_wrapper::glBindBuffer(gl::GL_ARRAY_BUFFER, vbo);
        gl_wrapper::glBufferData(
            gl::GL_ARRAY_BUFFER,
            (quad.len() * mem::size_of::<f32>()) as gl::GLsizeiptr,
            quad.as_ptr() as *const _,
            gl::GL_STATIC_DRAW,
        );
        gl_wrapper::glEnableVertexAttribArray(0);
        gl_wrapper::glVertexAttribPointer(
            0 as _,
            3,
            gl::GL_FLOAT,
            gl::GL_FALSE as gl::GLboolean,
            0,
            core::ptr::null(),
        );

        Ok(Quad { program })
    }

    fn update(&mut self, frame: u64) {}

    fn render(&self, frame: u64) {
        gl_wrapper::glUseProgram(self.program);

        let location =
            gl_wrapper::glGetUniformLocation(self.program, "frame\0".as_ptr() as *const _);
        gl_wrapper::glUniform1f(location, frame as f32);

        unsafe {
            gl::glClearColor(0.0, 0.0, 0.0, 1.0);
            gl::glClear((gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT) as gl::GLbitfield);
            gl::glDrawArrays(gl::GL_TRIANGLES, 0, 6);
        }
    }
}
