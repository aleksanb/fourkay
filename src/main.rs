#![feature(lang_items, start)]
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
use crate::bindings::Xlib::Atom;
use crate::shitty::xlib_events_ready;
use core::ptr::{null, null_mut};

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
    main().unwrap();
    0
}

macro_rules! intern_atom {
    ($display:ident, $atom_name:ident) => {{
        let atom_str = concat!(stringify!($atom_name), "\0");
        let atom = Xlib::XInternAtom(
            $display,
            atom_str.as_ptr() as *const _,
            Xlib_constants::False,
        );
        println!(
            concat!("Interned atom: ", stringify!($atom_name), ": %d\n\0"),
            atom
        );
        atom
    }};
}

fn main() -> Result<isize, ()> {
    unsafe {
        let display = Xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            println!("Couldn't set up display\n\0");
            return Err(());
        }

        let glx_display = display as *mut glx::Display;

        let mut glx_major = 0;
        let mut glx_minor = 0;
        let glx_result = glx::glXQueryVersion(glx_display, &mut glx_major, &mut glx_minor);
        println!(
            "glX version: Major: %d, minor: %d, result: %d\n\0",
            glx_major, glx_minor, glx_result
        );

        let default_screen = Xlib::XDefaultScreen(display);
        println!("default_screen: %d\n\0", default_screen);

        let mut attribute_list: &mut [libc::c_int] = &mut [
            glx::GLX_RGBA as libc::c_int,
            glx::GLX_RED_SIZE as libc::c_int,
            8,
            glx::GLX_GREEN_SIZE as libc::c_int,
            8,
            glx::GLX_BLUE_SIZE as libc::c_int,
            8,
            glx::None as libc::c_int,
        ];

        let visual_info =
            glx::glXChooseVisual(glx_display, default_screen, attribute_list.as_mut_ptr());
        println!("Visual info %p\n\0", visual_info as *const libc::c_char);

        let root_window = Xlib::XRootWindow(display, (*visual_info).screen);
        println!("Root window: %p\n\0", root_window as *const libc::c_char);

        let color_map = Xlib::XCreateColormap(
            display,
            root_window,
            (*visual_info).visual as *mut Xlib::Visual,
            Xlib_constants::AllocNone,
        );
        println!("Color map: %lu\n\0", color_map);

        let window = Xlib::XCreateSimpleWindow(
            display,
            root_window,
            0,
            0,
            //window_attributes.width as libc::c_uint,
            //window_attributes.height as libc::c_uint,
            1920,
            1080,
            0,
            0,
            0,
        );
        println!("Window: %lu\n\0", window);

        Xlib::XMapWindow(display, window);

        // // Hook close requests.
        let wm_protocols_atom = intern_atom!(display, WM_PROTOCOLS);
        let wm_delete_window_atom = intern_atom!(display, WM_DELETE_WINDOW);

        let _net_wm_state_atom = intern_atom!(display, _NET_WM_STATE);
        let _net_wm_state_fullscreen_atom = intern_atom!(display, _NET_WM_STATE_FULLSCREEN);

        let _net_wm_allowed_atom = intern_atom!(display, _NET_WM_ALLOWED_ACTIONS);
        let _net_wm_action_fullscreen_atom = intern_atom!(display, _NET_WM_ACTION_FULLSCREEN);
        let wm_a_atom = intern_atom!(display, ATOM);

        let mut protocols = [wm_delete_window_atom];
        Xlib::XSetWMProtocols(
            display,
            window,
            protocols.as_mut_ptr(),
            protocols.len() as libc::c_int,
        );

        let _NET_WM_STATE_REMOVE = 0; /* remove/unset property */
        let _NET_WM_STATE_ADD = 1; /* add/set property */
        let _NET_WM_STATE_TOGGLE = 2; /* toggle property  */

        Xlib::XChangeProperty(
            display,
            window,
            _net_wm_allowed_atom,
            wm_a_atom,
            32,
            Xlib::PropModeReplace as libc::c_int,
            &_net_wm_action_fullscreen_atom as *const libc::c_ulong as *const libc::c_uchar,
            1,
        );
        Xlib::XChangeProperty(
            display,
            window,
            _net_wm_state_atom,
            wm_a_atom,
            32,
            Xlib::PropModeReplace as libc::c_int,
            &_net_wm_state_fullscreen_atom as *const libc::c_ulong as *const libc::c_uchar,
            1,
        );

        let gl_context = glx::glXCreateContext(
            glx_display,
            visual_info,
            ptr::null_mut(),
            gl::GL_TRUE as i32,
        );
        println!("GL Context: %p\n\0", gl_context as *const libc::c_char);
        glx::glXMakeCurrent(glx_display, window, gl_context);

        gl::glEnable(gl::GL_DEPTH_TEST);
        gl_wrapper::load_extensions();

        let gl_version = gl_wrapper::glGetString(gl::GL_VERSION);
        println!("Version: %s\n\0", gl_version as *const libc::c_char);

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
    let mut raymarcher = programs::Quad::new()?;
    unsafe {
        let mut window_attributes: Xlib::XWindowAttributes = mem::uninitialized();
        Xlib::XGetWindowAttributes(display, window, &mut window_attributes);
        raymarcher.resize(window_attributes.width, window_attributes.height);
        println!(
            "Width inner window %d, height %d\n\0",
            window_attributes.width, window_attributes.height
        );
    }

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

                println!("event.type = %d\n\0", event.type_);
                match event.type_ {
                    Xlib_constants::Expose => {
                        println!("Window attributes!\n\0");
                        unsafe {
                            let mut window_attributes: Xlib::XWindowAttributes =
                                mem::uninitialized();
                            Xlib::XGetWindowAttributes(display, window, &mut window_attributes);
                            gl::glViewport(0, 0, window_attributes.width, window_attributes.height);
                        }
                    }
                    Xlib_constants::ClientMessage => {
                        println!("ClientMessage\n\0");
                        let xclient = event.xclient;
                        if xclient.message_type == wm_protocols_atom && xclient.format == 32 {
                            let protocol = xclient.data.l.as_ref()[0] as Xlib::Atom;
                            if protocol == wm_delete_window_atom {
                                return Ok(());
                            }
                        }
                        println!("Received event type of %d\n\0", xclient.message_type);
                    }
                    Xlib_constants::KeyPress => {
                        println!("Keyboard was pressed\n\0");
                    }
                    _ => (),
                }
            }
        }
    }
}
