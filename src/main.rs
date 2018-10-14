#![feature(lang_items, start)]
#![feature(dbg_macro)]
//#![no_std]
#![feature(alloc_system)]
extern crate alloc_system;

// extern crate gleam;
//extern crate glx;
extern crate libc;
extern crate x11_dl;

use core::panic::PanicInfo;
use gleam::gl;
use std::ffi;
use std::mem;
use std::ptr;
use x11_dl::{glx, xlib};

use std::rc::Rc;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    // const HELLO: &'static str = "Hello, world!\n\0";
    // unsafe {
    //     libc::write(libc::STDOUT_FILENO, HELLO.as_ptr() as *const _, HELLO.len());
    // }
    //let shader = unsafe {
    //gl::CreateShader(SHADER);
    //};
    //let vertex_shader = unsafe {
    //    gl::CreateShader(gl::TERTEX_SHADER);
    //};

    let xlib = xlib::Xlib::open().unwrap();
    let glx = glx::Glx::open().unwrap();

    //let display = unsafe{x11::xlib::XOpenDisplay(std::ptr::null());};
    unsafe {
        let display = (xlib.XOpenDisplay)(std::ptr::null());
        if display.is_null() {
            println!("fuck");
        }
        dbg!(display);

        let mut visual_attributes: Vec<libc::c_int> = vec![
            glx::GLX_X_RENDERABLE,
            1,
            glx::GLX_DRAWABLE_TYPE,
            glx::GLX_WINDOW_BIT,
            glx::GLX_RENDER_TYPE,
            glx::GLX_RGBA_BIT,
            glx::GLX_X_VISUAL_TYPE,
            glx::GLX_TRUE_COLOR,
            glx::GLX_RED_SIZE,
            8,
            glx::GLX_GREEN_SIZE,
            8,
            glx::GLX_BLUE_SIZE,
            8,
            glx::GLX_ALPHA_SIZE,
            8,
            glx::GLX_DEPTH_SIZE,
            24,
            glx::GLX_STENCIL_SIZE,
            8,
            glx::GLX_DOUBLEBUFFER,
            1,
            glx::GLX_NONE,
        ];

        let mut glx_major: libc::c_int = 0;
        let mut glx_minor: libc::c_int = 0;
        let result = (glx.glXQueryVersion)(display, &mut glx_major, &mut glx_minor);
        dbg!(glx_major);
        dbg!(glx_minor);
        dbg!(result);

        let default_screen = (xlib.XDefaultScreen)(display);
        dbg!(default_screen);

        let mut fb_count: libc::c_int = 10;
        let fb_config = (glx.glXChooseFBConfig)(
            display,
            default_screen,
            visual_attributes.as_ptr(),
            &mut fb_count,
        );
        dbg!(fb_count);
        dbg!(fb_config);
        if fb_config.is_null() {
            println!("Failed to retrieve a framebuffer config");
            return 1;
        }

        let visual = (glx.glXGetVisualFromFBConfig)(display, *fb_config);;
        dbg!(visual);

        let root_window = (xlib.XRootWindow)(display, (*visual).screen);
        dbg!(root_window);

        let color_map =
            (xlib.XCreateColormap)(display, root_window, (*visual).visual, xlib::AllocNone);
        dbg!(color_map);

        let mut set_window_attributes: xlib::XSetWindowAttributes = mem::uninitialized();
        set_window_attributes.colormap = color_map;
        set_window_attributes.background_pixel = (xlib.XWhitePixel)(display, (*visual).screen);
        set_window_attributes.event_mask = xlib::ExposureMask | xlib::KeyPressMask;

        let window: xlib::Window = (xlib.XCreateWindow)(
            display,
            root_window,
            0,
            0,
            1024,
            768,
            0,
            (*visual).depth,
            xlib::InputOutput as libc::c_uint,
            (*visual).visual,
            xlib::CWColormap | xlib::CWEventMask, // | xlib::CWBackPixel ,
            &mut set_window_attributes,
        );
        dbg!(window);

        // Show window.
        (xlib.XMapWindow)(display, window);
        let title = ffi::CString::new("fourkay").unwrap();
        (xlib.XStoreName)(display, window, title.as_ptr() as *mut libc::c_char);

        let gl_context = (glx.glXCreateContext)(display, visual, ptr::null_mut(), gl::TRUE as i32);
        dbg!(gl_context);
        dbg!(glx.glXMakeCurrent)(display, window, gl_context);

        let gl: Rc<gl::Gl> = gl::GlFns::load_with(|symbol: &str| {
            let symbol_as_cstring = ffi::CString::new(symbol.as_bytes()).unwrap();
            let address = symbol_as_cstring.as_ptr();
            (glx.glXGetProcAddress)(address as *const _).unwrap() as *const _
        });
        let gl: Rc<dyn gl::Gl> = gl::ErrorCheckingGl::wrap(gl);
        gl.enable(gl::DEPTH_TEST);

        let gl_version = gl.get_string(gl::VERSION);
        dbg!(gl_version);

        // Hook close requests.
        let wm_protocols_str = ffi::CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = ffi::CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols = (xlib.XInternAtom)(display, wm_protocols_str.as_ptr(), xlib::False);
        let wm_delete_window =
            (xlib.XInternAtom)(display, wm_delete_window_str.as_ptr(), xlib::False);

        let mut protocols = [wm_delete_window];

        (xlib.XSetWMProtocols)(
            display,
            window,
            protocols.as_mut_ptr(),
            protocols.len() as libc::c_int,
        );

        // Main loop.
        let mut event: xlib::XEvent = mem::uninitialized();
        let mut window_attributes: xlib::XWindowAttributes = mem::uninitialized();
        let mut count = 0;

        loop {
            (xlib.XNextEvent)(display, &mut event);

            match event.get_type() {
                xlib::Expose => {
                    (xlib.XGetWindowAttributes)(display, window, &mut window_attributes);
                    dbg!(window_attributes);
                    gl.viewport(0, 0, window_attributes.width, window_attributes.height);

                    setup(&*gl);
                    (glx.glXSwapBuffers)(display, window);
                }
                xlib::ClientMessage => {
                    dbg!("We client message now");
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == wm_delete_window {
                            break;
                        }
                    }
                }
                xlib::KeyPress => {
                    if count % 2 == 0 {
                        red(&*gl);
                    } else {
                        blue(&*gl);
                    }
                    (glx.glXSwapBuffers)(display, window);
                    count += 1;
                    dbg!(event);
                }
                _ => (),
            }
        }

        // Shut down.
        //(glx.glXMakeCurrent)(display, glx::GLX_NONE as _, ptr::null_mut());
        (glx.glXDestroyContext)(display, gl_context);
        (xlib.XDestroyWindow)(display, window);
        (xlib.XCloseDisplay)(display);
    }
    0
}

enum ShaderType {
    VertexShader(String),
    FragmentShader(String),
}

fn load_shader(gl: &dyn gl::Gl, shader_type: ShaderType) -> Result<gl::GLuint, ()> {
    let shader = gl.create_shader(match shader_type {
        ShaderType::VertexShader(_) => gl::VERTEX_SHADER,
        ShaderType::FragmentShader(_) => gl::FRAGMENT_SHADER,
    });
    let shader_cstring = ffi::CString::new(match shader_type {
        ShaderType::VertexShader(s) | ShaderType::FragmentShader(s) => s,
    })
    .unwrap();

    gl.shader_source(shader, &[shader_cstring.as_bytes()]);
    gl.compile_shader(shader);

    let mut is_compiled: Vec<gl::GLint> = vec![0; 2];
    unsafe {
        gl.get_shader_iv(shader, gl::COMPILE_STATUS, &mut is_compiled);
    }

    if is_compiled[0] as gl::GLboolean == gl::FALSE {
        dbg!("Failure");

        // let mut max_length: Vec<gl::GLint> = vec![10];
        // unsafe {
        //     gl.get_shader_iv(shader, gl::INFO_LOG_LENGTH, &mut max_length);
        // }
        // dbg!(max_length);
        //let error_log = vec![0; max_length];
        let log = gl.get_shader_info_log(shader);
        dbg!(log);
        return Err(());
    }

    Ok(shader)
}

static VERTEX_SHADER: &'static str = "
#version 130
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
";

static FRAGMENT_SHADER: &'static str = "
#version 130
out vec4 outColor;

void main() {
    outColor = vec4(1.0, 0.0, 1.0, 1.0);
}
";

fn setup(gl: &dyn gl::Gl) {
    let vertex_arrays = gl.gen_vertex_arrays(1);
    let vao = vertex_arrays[0];
    gl.bind_vertex_array(vao);

    let vertices = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
    let buffers = gl.gen_buffers(1);
    let vbo = buffers[0];
    gl.bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(gl, gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);

    let fragment_shader =
        load_shader(gl, ShaderType::FragmentShader(FRAGMENT_SHADER.to_string())).unwrap();
    let vertex_shader =
        load_shader(gl, ShaderType::VertexShader(VERTEX_SHADER.to_string())).unwrap();

    let program = gl.create_program();
    gl.attach_shader(program, fragment_shader);
    gl.attach_shader(program, vertex_shader);

    gl.bind_frag_data_location_indexed(program, 0, 0, "outColor");
    gl.link_program(program);
    gl.use_program(program);

    let mut program_status = vec![0];
    unsafe {
        gl.get_program_iv(program, gl::LINK_STATUS, &mut program_status);
    }
    if (dbg!(program_status[0]) as gl::GLboolean) == gl::FALSE {
        dbg!("Failure");

        // let mut max_length: Vec<gl::GLint> = vec![10];
        // unsafe {
        //     gl.get_program_iv(program, gl::INFO_LOG_LENGTH, &mut max_length);
        // }
        // dbg!(max_length);
        //let error_log = vec![0; max_length];
        let log = gl.get_program_info_log(program);
        dbg!(log);
    }

    let pos_attrib = gl.get_attrib_location(program, "position");
    dbg!(pos_attrib);
    gl.enable_vertex_attrib_array(pos_attrib as _);
    gl.vertex_attrib_pointer(pos_attrib as _, 2, gl::FLOAT, false, 0, 0);

    render(gl);
}

fn red(gl: &dyn gl::Gl) {
    gl.clear_color(1.0, 0.0, 0.0, 1.0);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}

fn blue(gl: &dyn gl::Gl) {
    gl.clear_color(0.0, 0.0, 1.0, 1.0);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}

fn render(gl: &dyn gl::Gl) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gl.draw_arrays(gl::TRIANGLES, 0, 3);
}

// fn actual_main(window: impl WindowMethods) {
//     //window.gl.
// }

//  #[link(name = "GL")]
//  extern "C" {
//      fn glFinish();
//  }

// #[link(name = "GLX")]
// extern "C" {
//     fn XOpenDisplay(display_name: *const libc::c_char);
// }

// #[link(name = "X11")]
// extern "C" {
//     fn XOpenDisplay(display_name: *const libc::c_char) -> *mut libc::c_void;
//     fn XDefaultRootWindow(_1: *mut libc::c_void) -> libc::c_ulong;
// }

// #[panic_handler]
// fn panic(_panic: &PanicInfo<'_>) -> ! {
//     loop {}
// }
//
// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}
