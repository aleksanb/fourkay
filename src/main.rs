#![feature(lang_items, start)]
#![no_std]

#[cfg(println)]
#[macro_use]
mod shitty;
#[cfg(println)]
use self::shitty::Prepare;

#[cfg(not(println))]
macro_rules! println {
    ($($val:expr),*) => {};
}

//extern crate alloc;
//use alloc::vec::Vec;

//#![macro_use] extern crate alloc;
//use alloc::string::ToString;

use core::panic::PanicInfo;
// use gleam::gl;
//use std::ffi;
use core::mem;
use core::ptr;
//use core::rc::Rc;
use core::ffi;

mod bindings;

use self::bindings::{gl, glx, Xlib, Xlib_constants};
use libc::c_long;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

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

    //let display = unsafe{x11::Xlib::XOpenDisplay(std::ptr::null());};
    unsafe {
        let display = Xlib::XOpenDisplay(ptr::null());
        // let old_display: *mut OldDisplay = mem::transmute(display);
        //let display = (Xlib.XOpenDisplay)(std::ptr::null());
        if display.is_null() {
            println!("fuck\n\0");
        }

        println!("Got displayyyyyyyy: %p\n\0", display as *const libc::c_char);

        let glx_display: *mut glx::Display = mem::transmute(display);

        let mut glx_major: libc::c_int = 0;
        let mut glx_minor: libc::c_int = 0;
        let result = glx::glXQueryVersion(glx_display, &mut glx_major, &mut glx_minor);
        println!("%d\n\0", glx_major);
        println!("%d\n\0", glx_minor);
        println!("%d\n\0", result);

        let default_screen = Xlib::XDefaultScreen(display);
        println!("default_screen: %d\n\0", default_screen);

        let visual_attributes: &[libc::c_int] = &[
            glx::GLX_RGBA as libc::c_int,
            glx::GLX_DEPTH_SIZE as libc::c_int,
            24,
            glx::GLX_DOUBLEBUFFER as libc::c_int,
            glx::GLX_NONE as libc::c_int,
        ];
        let visual = glx::glXChooseVisual(glx_display, 0, visual_attributes.as_ptr() as *mut _);
        println!("got visual: %p\n\0", visual as *const libc::c_char);

        // let visual_attributes: &[libc::c_int] = &[
        //     glx::GLX_X_RENDERABLE as libc::c_int,
        //     1,
        //     glx::GLX_DRAWABLE_TYPE as libc::c_int,
        //     glx::GLX_WINDOW_BIT as libc::c_int,
        //     glx::GLX_RENDER_TYPE as libc::c_int,
        //     glx::GLX_RGBA_BIT as libc::c_int,
        //     glx::GLX_X_VISUAL_TYPE as libc::c_int,
        //     glx::GLX_TRUE_COLOR as libc::c_int,
        //     glx::GLX_RED_SIZE as libc::c_int,
        //     8,
        //     glx::GLX_GREEN_SIZE as libc::c_int,
        //     8,
        //     glx::GLX_BLUE_SIZE as libc::c_int,
        //     8,
        //     glx::GLX_ALPHA_SIZE as libc::c_int,
        //     8,
        //     glx::GLX_DEPTH_SIZE as libc::c_int,
        //     24,
        //     glx::GLX_STENCIL_SIZE as libc::c_int,
        //     8,
        //     glx::GLX_DOUBLEBUFFER as libc::c_int,
        //     1,
        //     glx::GLX_NONE as libc::c_int,
        // ];

        // let mut fb_count: libc::c_int = 10;
        // let fb_config = glx::glXChooseFBConfig(
        //     glx_display,
        //     default_screen,
        //     visual_attributes.as_ptr(),
        //     &mut fb_count,
        // );

        // println!("fb_count: %d\n\0", fb_count);
        // println!("fb_config: %p\n\0", fb_config as *const libc::c_char);
        // if fb_config.is_null() {
        //     println!("Failed to retrieve a framebuffer config\n\0");
        //     return 1;
        // }

        // let visual = glx::glXGetVisualFromFBConfig(glx_display, *fb_config);;
        // println!("%d\n\0", visual as * const libc::c_char);

        let root_window = Xlib::XRootWindow(display, (*visual).screen);
        println!("Root window: %p\n\0", root_window as *const libc::c_char);

        let color_map = Xlib::XCreateColormap(
            display,
            root_window,
            mem::transmute((*visual).visual),
            Xlib_constants::AllocNone,
        );
        println!("Color map: %lu\n\0", color_map);

        let mut set_window_attributes: Xlib::XSetWindowAttributes = mem::uninitialized();
        set_window_attributes.colormap = color_map;
        set_window_attributes.event_mask =
            Xlib_constants::ExposureMask | Xlib_constants::KeyPressMask;
        set_window_attributes.background_pixel = Xlib::XWhitePixel(display, (*visual).screen);

        let window: Xlib::Window = Xlib::XCreateWindow(
            display,
            root_window,
            0,
            0,
            1024,
            768,
            0,
            (*visual).depth,
            Xlib::InputOutput as libc::c_uint,
            mem::transmute((*visual).visual),
            Xlib_constants::CWColormap | Xlib_constants::CWEventMask | Xlib_constants::CWBackPixel,
            &mut set_window_attributes,
        );
        println!("Window: %lu\n\0", window);

        // Show window.
        Xlib::XMapWindow(display, window);
        let title = "fourkay\0";
        Xlib::XStoreName(display, window, title.as_ptr() as *mut _);

        let gl_context =
            glx::glXCreateContext(glx_display, visual, ptr::null_mut(), gl::GL_TRUE as i32);
        println!("GL Context: %p\n\0", gl_context as *const libc::c_char);
        glx::glXMakeCurrent(glx_display, window, gl_context);

        gl::glEnable(gl::GL_DEPTH_TEST);

        //loop{}

        // const GLGETSTRING_NAME: &'static str = "glGetStringi\0";
        // let address = glx::glXGetProcAddress(GLGETSTRING_NAME.as_ptr());
        // let glGetStringi: gl::PFNGLGETSTRINGIPROC = mem::transmute(address);
        // let glGetStringi = glGetStringi.unwrap();


        // let version = glGetStringi(gl::GL_VERSION, 0);
        // println!("Version: %s\n\0", version as *const libc::c_char);




        //let version = ffi::CString::from_raw(version);
        // dbg!(version);
        // let gl: Rc<gl::Gl> = gl::GlFns::load_with(|symbol: &str| {
        //     let symbol_as_cstring = ffi::CString::new(symbol.as_bytes()).unwrap();
        //     let address = symbol_as_cstring.as_ptr();
        //     glx::glXGetProcAddress(address as *const _).unwrap() as *const _
        // });
        // let gl: Rc<dyn gl::Gl> = gl::ErrorCheckingGl::wrap(gl);
        // gl.enable(gl::DEPTH_TEST);

        // let gl_version = gl.get_string(gl::VERSION);
        // dbg!(gl_version);

        // // Hook close requests.
        // let wm_protocols_str = ffi::CString::new("WM_PROTOCOLS").unwrap();
        // let wm_delete_window_str = ffi::CString::new("WM_DELETE_WINDOW").unwrap();

        // let wm_protocols =
        //     Xlib::XInternAtom(display, wm_protocols_str.as_ptr(), Xlib_constants::False);
        // let wm_delete_window = Xlib::XInternAtom(
        //     display,
        //     wm_delete_window_str.as_ptr(),
        //     Xlib_constants::False,
        // );

        // let mut protocols = [wm_delete_window];

        // Xlib::XSetWMProtocols(
        //     display,
        //     window,
        //     protocols.as_mut_ptr(),
        //     protocols.len() as libc::c_int,
        // );

        // // Main loop.
        let mut event: Xlib::XEvent = mem::uninitialized();
        let mut window_attributes: Xlib::XWindowAttributes = mem::uninitialized();
        let mut count = 0;

        loop {
            Xlib::XNextEvent(display, &mut event);

            match event.type_.as_ref() {
                &Xlib_constants::Expose => {
                    Xlib::XGetWindowAttributes(display, window, &mut window_attributes);
                    gl::glViewport(0, 0, window_attributes.width, window_attributes.height);
                    glx::glXSwapBuffers(glx_display, window);
                }
                &Xlib_constants::ClientMessage => {
                    //dbg!("We client message now");
                    // let xclient = Xlib::XClientMessageEvent::from(event);

                    // if xclient.message_type == wm_protocols && xclient.format == 32 {
                    //     let protocol = xclient.data.get_long(0) as Xlib::Atom;

                    //     if protocol == wm_delete_window {
                    //         break;
                    //    }
                    // }
                }
                &Xlib_constants::KeyPress => {
                    // if count % 2 == 0 {
                    //     red(&*gl);
                    // } else {
                    //     blue(&*gl);
                    // }
                    // (glx.glXSwapBuffers)(display, window);
                    // count += 1;
                    // dbg!(event);
                }
                _ => (),
            }
        }

        // // Shut down.
        // //(glx.glXMakeCurrent)(display, glx::GLX_NONE as _, ptr::null_mut());
        // glx::glXDestroyContext(glx_display, gl_context);
        // Xlib::XDestroyWindow(display, window);
        // Xlib::XCloseDisplay(display);
    }
    0
}

/*enum ShaderType {
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
 in vec3 position;

 void main() {
     gl_Position = vec4(vec2(position), 0.0, 1.0);
 }
 ";

static FRAGMENT_SHADER: &'static str = "
 #version 130

 void main() {
     gl_FragColor = vec4(gl_FragCoord.x / 1024.0, 0.0, gl_FragCoord.y / 768.0, 1.0);
 }
 ";

fn setup(gl: &dyn gl::Gl) {
    let vertex_arrays = gl.gen_vertex_arrays(1);
    let vao = vertex_arrays[0];
    dbg!(vao);
    gl.bind_vertex_array(vao);

    let quad: Vec<f32> = vec![0.0, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];
    // let quad: Vec<f32> = vec![-1.0, 1.0, 0.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0];

    let vbo = gl.gen_buffers(1)[0];
    gl.bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(gl, gl::ARRAY_BUFFER, &quad, gl::STATIC_DRAW);

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer(0 as _, 3, gl::FLOAT, false, 0, 0);

    let fragment_shader =
        load_shader(gl, ShaderType::FragmentShader(FRAGMENT_SHADER.to_string())).unwrap();
    let vertex_shader =
        load_shader(gl, ShaderType::VertexShader(VERTEX_SHADER.to_string())).unwrap();

    let program = gl.create_program();
    gl.attach_shader(program, fragment_shader);
    gl.attach_shader(program, vertex_shader);

    gl.link_program(program);

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

    gl.use_program(program);

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
*/
