#![feature(lang_items, start, duration_as_u128)]
#![no_std]

#[macro_use]
mod shitty;
use self::shitty::{gl_utils, gl_wrapper, println::*};

use self::programs::Program;
use core::mem;
use core::panic::PanicInfo;
use core::ptr;

mod bindings;
mod programs;

use self::bindings::{gl, glx, Xlib, Xlib_constants};

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    if let Err(()) = main() {
        println!("Program failed with error code %d\n\0", 1u32);
        return 1;
    }

    0
}

fn main() -> Result<isize, ()> {
    unsafe {
        let display = Xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            println!("Coudln't set up display\n\0");
            return Err(());
        }

        let glx_display = display as *mut glx::Display;

        let mut glx_major: libc::c_int = 0;
        let mut glx_minor: libc::c_int = 0;
        let glx_result = glx::glXQueryVersion(glx_display, &mut glx_major, &mut glx_minor);
        println!(
            "glX version: Major: %d, minor: %d, result: %d\n\0",
            glx_major, glx_minor, glx_result
        );

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
            (*visual).visual as *mut Xlib::Visual,
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
            (*visual).visual as *mut Xlib::Visual,
            Xlib_constants::CWColormap | Xlib_constants::CWEventMask | Xlib_constants::CWBackPixel,
            &mut set_window_attributes,
        );
        println!("Window: %lu\n\0", window);

        Xlib::XMapWindow(display, window);
        let title = "fourkay\0";
        Xlib::XStoreName(display, window, title.as_ptr() as *mut _);

        let gl_context =
            glx::glXCreateContext(glx_display, visual, ptr::null_mut(), gl::GL_TRUE as i32);
        println!("GL Context: %p\n\0", gl_context as *const libc::c_char);
        glx::glXMakeCurrent(glx_display, window, gl_context);

        gl::glEnable(gl::GL_DEPTH_TEST);
        gl_wrapper::load_extensions();

        let gl_version = gl_wrapper::glGetString(gl::GL_VERSION);
        println!("Version: %s\n\0", gl_version as *const libc::c_char);

        // // Hook close requests.
        let wm_protocols_str = "WM_PROTOCOLS\0";
        let wm_protocols_atom = Xlib::XInternAtom(
            display,
            wm_protocols_str.as_ptr() as *const _,
            Xlib_constants::False,
        );
        println!("WM Protocol atom? %d\n\0", wm_protocols_atom);

        let wm_delete_window_str = "WM_DELETE_WINDOW\0";
        let wm_delete_window_atom = Xlib::XInternAtom(
            display,
            wm_delete_window_str.as_ptr() as *const _,
            Xlib_constants::False,
        );
        println!("Delete window atom? %d\n\0", wm_delete_window_atom);

        let mut protocols = [wm_delete_window_atom];
        Xlib::XSetWMProtocols(
            display,
            window,
            protocols.as_mut_ptr(),
            protocols.len() as libc::c_int,
        );

        main_loop(display, window, wm_protocols_atom, wm_delete_window_atom)?;

        if glx::glXMakeCurrent(glx_display, glx::GLX_NONE.into(), ptr::null_mut()) == 0 {
            return Err(());
        };

        glx::glXDestroyContext(glx_display, gl_context);
        if Xlib::XDestroyWindow(display, window) == 0 {
            return Err(());
        };

        if Xlib::XCloseDisplay(display) == 0 {
            return Err(());
        };
    }

    Ok(0)
}

fn main_loop(
    display: *mut Xlib::_XDisplay,
    window: Xlib::Window,
    wm_protocols_atom: Xlib::Atom,
    wm_delete_window_atom: Xlib::Atom,
) -> Result<(), ()> {
    //let mut quad_program = programs::Quad::new()?;
    let mut raymarcher = programs::Raymarcher::new()?;

    const FRAMES_PER_SECOND: u64 = 60;
    const FRAME_LENGTH_MILLISECONDS: u64 = 1_000 / FRAMES_PER_SECOND;
    const FRAME_LENGTH_DURATION: core::time::Duration =
        core::time::Duration::from_millis(FRAME_LENGTH_MILLISECONDS);

    let mut current_frame = 0;

    let mut current_time = shitty::time::now();
    let mut previous_time = shitty::time::now();

    let mut delta_time = core::time::Duration::new(0, 0);

    loop {
        shitty::time::update(&mut current_time);
        let delta_since_last_wake = shitty::time::subtract(&current_time, &previous_time);
        delta_time += delta_since_last_wake;
        previous_time = current_time;

        while delta_time >= FRAME_LENGTH_DURATION {
            //quad_program.update(current_frame);
            raymarcher.update(current_frame);

            delta_time -= FRAME_LENGTH_DURATION;
            current_frame += 1;
        }
        raymarcher.render(current_frame);

        unsafe {
            glx::glXSwapBuffers(display as *mut bindings::glx::_XDisplay, window);

            let events_pending = Xlib::XPending(display);
            for _ in 0..events_pending {
                let mut event: Xlib::XEvent = mem::uninitialized();
                Xlib::XNextEvent(display, &mut event);

                println!("event.type = %d\n\0", event.type_.as_ref());
                match event.type_.as_ref() {
                    &Xlib_constants::Expose => {
                        println!("Window attributes!\n\0");
                        unsafe {
                            let mut window_attributes: Xlib::XWindowAttributes =
                                mem::uninitialized();
                            Xlib::XGetWindowAttributes(display, window, &mut window_attributes);
                            gl::glViewport(0, 0, window_attributes.width, window_attributes.height);
                        }
                    }
                    &Xlib_constants::ClientMessage => {
                        println!("ClientMessage\n\0");
                        let xclient = event.xclient.as_ref();
                        if xclient.message_type == wm_protocols_atom && xclient.format == 32 {
                            let protocol = xclient.data.l.as_ref()[0] as Xlib::Atom;
                            if protocol == wm_delete_window_atom {
                                return Ok(());
                            }
                        }
                    }
                    &Xlib_constants::KeyPress => {
                        println!("Keyboard was pressed\n\0");
                    }
                    _ => (),
                }
            }
        }
    }
}
