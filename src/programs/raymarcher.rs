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
}

impl Program for Raymarcher {
    fn new() -> Result<Self, ()> {
        let fragment_shader =
            gl_utils::create_shader(&gl_utils::ShaderType::FragmentShader(FRAGMENT_SHADER))?;
        let vertex_shader =
            gl_utils::create_shader(&gl_utils::ShaderType::VertexShader(VERTEX_SHADER))?;
        let program = gl_utils::create_program(fragment_shader, vertex_shader)?;
        gl_wrapper::glUseProgram(program);

        const NUM_VERTEX_ARRAYS: gl::GLsizei = 1;
        let vertex_arrays: &mut [gl::GLuint] = &mut [0; NUM_VERTEX_ARRAYS as usize];
        gl_wrapper::glGenVertexArrays(NUM_VERTEX_ARRAYS, vertex_arrays.as_ptr() as *mut _);
        let vao = vertex_arrays[0];
        println!("vao = %d\n\0", vao);
        gl_wrapper::glBindVertexArray(vao);

        const NUM_BUFFERS: gl::GLsizei = 1;
        let buffers: &mut [gl::GLuint] = &mut [0; NUM_BUFFERS as usize];
        gl_wrapper::glGenBuffers(NUM_BUFFERS, buffers.as_ptr() as *mut _);
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

        Ok(Self { program })
    }

    fn update(&mut self, _frame: u64) {}

    fn render(&self, frame: u64) {
        gl_wrapper::glUseProgram(self.program);

        let uniform_frame =
            gl_wrapper::glGetUniformLocation(self.program, "frame\0".as_ptr() as *const _);
        gl_wrapper::glUniform1f(uniform_frame, frame as f32 / 4.0);

        let uniform_eye =
            gl_wrapper::glGetUniformLocation(self.program, "eye\0".as_ptr() as *const _);
        gl_wrapper::glUniform3f(uniform_eye, 0.0, 0.0, 10.0);

        let uniform_forward =
            gl_wrapper::glGetUniformLocation(self.program, "forward\0".as_ptr() as *const _);
        gl_wrapper::glUniform3f(uniform_forward, 0.0, 0.0, -1.0);

        unsafe {
            gl::glClearColor(0.0, 0.0, 0.0, 1.0);
            gl::glClear((gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT) as gl::GLbitfield);
            gl::glDrawArrays(gl::GL_TRIANGLES, 0, 6);
        }
    }
}
