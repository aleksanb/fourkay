use crate::bindings::gl;
use crate::shitty::{gl_wrapper, println::*};

pub enum ShaderType {
    VertexShader(&'static str),
    FragmentShader(&'static str),
}

pub fn create_shader(shader_type: &ShaderType) -> Result<gl::GLuint, ()> {
    let gl_shader_type = match shader_type {
        ShaderType::VertexShader(_) => gl::GL_VERTEX_SHADER,
        ShaderType::FragmentShader(_) => gl::GL_FRAGMENT_SHADER,
    };

    let shader_body = match shader_type {
        ShaderType::VertexShader(shader_body) | ShaderType::FragmentShader(shader_body) => {
            shader_body.as_bytes().as_ptr() as *const libc::c_char
        }
    };

    let shader_strings = &[shader_body];
    let shader_id = gl_wrapper::glCreateShaderProgramv(gl_shader_type, 1, shader_strings as *const *const gl::GLchar);
    return Ok(shader_id);

    let shader_id = gl_wrapper::glCreateShader(match shader_type {
        ShaderType::VertexShader(_) => gl::GL_VERTEX_SHADER,
        ShaderType::FragmentShader(_) => gl::GL_FRAGMENT_SHADER,
    });

    let shader_body = match shader_type {
        ShaderType::VertexShader(shader_body) | ShaderType::FragmentShader(shader_body) => {
            shader_body.as_bytes().as_ptr() as *const libc::c_char
        }
    };

    let shader_strings = &[shader_body];

    gl_wrapper::glShaderSource(
        shader_id,
        1,
        shader_strings as *const *const gl::GLchar,
        core::ptr::null(),
    );
    gl_wrapper::glCompileShader(shader_id);

    if cfg!(feature = "error-handling") {
        let mut is_compiled: gl::GLint = 0;
        gl_wrapper::glGetShaderiv(shader_id, gl::GL_COMPILE_STATUS, &mut is_compiled);

        if is_compiled as u32 == gl::GL_FALSE {
            println!("Ok we failed compiling the shader\n\0");
            let mut max_length: gl::GLint = 1337;
            gl_wrapper::glGetShaderiv(shader_id, gl::GL_INFO_LOG_LENGTH, &mut max_length);
            println!("Max length is: %d\n\0", max_length);

            let buffer: &mut [libc::c_char] = &mut [0; 1024];
            gl_wrapper::glGetShaderInfoLog(
                shader_id,
                buffer.len() as gl::GLsizei,
                core::ptr::null_mut(),
                buffer.as_ptr() as *mut _,
            );
            println!("Error compiling shader: %s\n\0", buffer.as_ptr());
            return Err(());
        }

        println!("Successfully compiled shader #%d\n\0", shader_id);
    }
    Ok(shader_id)
}

pub fn create_program(
    fragment_shader: gl::GLuint,
    vertex_shader: gl::GLuint,
) -> Result<gl::GLuint, ()> {
    let program = gl_wrapper::glCreateProgram();
    gl_wrapper::glAttachShader(program, fragment_shader);
    gl_wrapper::glAttachShader(program, vertex_shader);
    gl_wrapper::glLinkProgram(program);

    if cfg!(feature = "error-handling") {
        let mut program_status = 0;
        gl_wrapper::glGetProgramiv(program, gl::GL_LINK_STATUS, &mut program_status);

        if program_status as u32 == gl::GL_FALSE {
            let mut max_length: gl::GLint = 0;
            gl_wrapper::glGetProgramiv(program, gl::GL_INFO_LOG_LENGTH, &mut max_length);
            println!("Max length is: %d\n\0", max_length);

            let buffer: &mut [libc::c_char] = &mut [0; 1024];
            gl_wrapper::glGetProgramInfoLog(
                program,
                buffer.len() as gl::GLsizei,
                core::ptr::null_mut(),
                buffer.as_ptr() as *mut _,
            );
            println!("Error compiling programs: %s\n\0", buffer.as_ptr());
            return Err(());
        }
    }

    println!("Successfully compiled programs #%d\n\0", program);
    Ok(program)
}
