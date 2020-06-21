#![feature(lang_items, start, raw_ref_op)]
#![no_std]
#![no_main]
// https://doc.rust-lang.org/1.19.0/reference/attributes.html#crate-only-attributes

#[macro_use]
mod shitty;
use self::shitty::{gl_wrapper, println::*};

use self::programs::Program;
use core::mem;
use core::ptr;

mod bindings;
mod programs;

use self::bindings::{gl, glx, Xlib, Xlib_constants};

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

extern "C" {
    // pub fn __4klang_render(arg1: *const libc::c_char) -> *const libc::c_char;

    //pub fn __4klang_envelope(arg1: *const libc::c_char) -> *const libc::c_float;
    //pub fn __4klang_note_buffer(arg1: *const libc::c_char) -> *const libc::c_int;
    //pub fn sound_initialize();
    //pub fn sound_play();
    //pub fn sound_stop();
}

//extern "C" {
//    extern void* __4klang_render(void*);
//    extern float __4klang_envelope_buffer;
//    extern int   __4klang_note_buffer;
//}

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

// void main(){gl_FragColor = vec4(1.0,0.5,0.5,.5);}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    const SAMPLE_RATE: u32 = 44100;
    const BPM: u32 = 120;
    const MAX_INSTRUMENTS: u32 = 4;
    const MAX_PATTERNS: u32 = 84;

    // let sound_buffer: *mut i8 = unsafe { mem::transmute(libc::malloc(1024 * 1024 * 30 * mem::size_of::<u8>())) };
    //let sound_buffer_position = sound_buffer as *const i8;
    //let sound_thread_stack = [0u8; 1024*1024];

    /*unsafe {
        __4klang_render(sound_buffer);
    }*/

    /*unsafe {
        let mut sound_spec = SDL::SDL_AudioSpec {
            freq: SAMPLE_RATE,
            format: SDL::AUDIO_S16SYS,
            channels: 2,
            silence: 0,
            samples: 4096,
            size : 0,
            callback:
        };
        SDL::SDL_LoadWAV_RW(sound_buffer, 0, &mut sound_spec as *mut SDL::SDL_AudioSpec);;
    }

    unsafe {
        //sound_initialize();
        //sound_play();

        SDL::SDL_OpenAudio(&mut sound_spec as *mut _, null());
    }*/

    //libc::clone()

    unsafe {
        let display = Xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            println!("Couldn't set up display\n\0");
            return 1;
        }

        let glx_display = display as *mut glx::Display;

        // let mut glx_major = 0;
        // let mut glx_minor = 0;
        //let glx_result = glx::glXQueryVersion(glx_display, &mut glx_major, &mut glx_minor);
        //println!(
        //"glX version: Major: %d, minor: %d, result: %d\n\0",
        //glx_major, glx_minor, glx_result
        //);

        let default_screen = Xlib::XDefaultScreen(display);
        println!("default_screen: %d\n\0", default_screen);

        let attribute_list = &mut [
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

        let mut set_window_attributes: Xlib::XSetWindowAttributes =
            mem::MaybeUninit::uninit().assume_init();
        set_window_attributes.colormap = color_map;
        set_window_attributes.event_mask =
            Xlib_constants::ExposureMask | Xlib_constants::KeyPressMask;
        set_window_attributes.background_pixel = Xlib::XWhitePixel(display, (*visual_info).screen);
        let window_flags_enabled =
            Xlib_constants::CWColormap | Xlib_constants::CWEventMask | Xlib_constants::CWBackPixel;

        let window = Xlib::XCreateWindow(
            display,
            root_window,
            0,
            0,
            1920,
            1080,
            0,
            (*visual_info).depth,
            Xlib::InputOutput as libc::c_uint,
            (*visual_info).visual as *mut Xlib::Visual,
            window_flags_enabled,
            &mut set_window_attributes,
        );

        /*let window = Xlib::XCreateSimpleWindow(
            display,
            root_window,
            0,
            0,
            1920,
            1080,
            0,
            0,
            0,
        );*/
        println!("Window: %lu\n\0", window);

        Xlib::XMapWindow(display, window);

        // // Hook close requests.
        //let wm_protocols_atom = intern_atom!(display, WM_PROTOCOLS);
        //let wm_delete_window_atom = intern_atom!(display, WM_DELETE_WINDOW);

        let _net_wm_state_atom = intern_atom!(display, _NET_WM_STATE);
        let _net_wm_state_fullscreen_atom = intern_atom!(display, _NET_WM_STATE_FULLSCREEN);

        let _net_wm_allowed_atom = intern_atom!(display, _NET_WM_ALLOWED_ACTIONS);
        let _net_wm_action_fullscreen_atom = intern_atom!(display, _NET_WM_ACTION_FULLSCREEN);
        let wm_a_atom = intern_atom!(display, ATOM);

        // let mut protocols = [wm_delete_window_atom];
        // Xlib::XSetWMProtocols(
        // display,
        // window,
        // protocols.as_mut_ptr(),
        // protocols.len() as libc::c_int,
        // );

        let _net_wm_state_remove = 0; /* remove/unset property */
        let _net_wm_state_add = 1; /* add/set property */
        let _net_wm_state_toggle = 2; /* toggle property  */

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

        #[cfg(feature = "error-handling")]
        {
            let gl_version = gl_wrapper::glGetString(gl::GL_VERSION);
            println!("Version: %s\n\0", gl_version as *const libc::c_char);
        }

        main_loop(display, window);

        #[cfg(feature = "error-handling")]
        {
            if glx::glXMakeCurrent(glx_display, glx::GLX_NONE.into(), ptr::null_mut()) == 0 {
                return 1;
            };

            glx::glXDestroyContext(glx_display, gl_context);
            if Xlib::XDestroyWindow(display, window) == 0 {
                return 1;
            };

            if Xlib::XCloseDisplay(display) == 0 {
                return 1;
            };
        }
    }

    0
}

static VERTEX_SHADER: &str = concat!(include_str!("shaders/quad-vertex.glsl"), "\0");
static BALLS_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/balls.glsl.out"), "\0");
static SOLID_FRAGMENT_SHADER: &str = "void main(){gl_FragColor = vec4(1.0, 0.5, 0.5, 0.5);}\0";

static FLOWERS_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/flower.glsl"), "\0");
static BLOBBY_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/blobby.glsl.out"), "\0");
static SNAKE_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/snake.glsl.out"), "\0");

fn main_loop(display: *mut Xlib::_XDisplay, window: Xlib::Window) -> Result<(), ()> {
    ///let mut kaleidoscope_shader = programs::Quad::new(BALLS_FRAGMENT_SHADER, VERTEX_SHADER)?;
    //let mut flower_shader = programs::Quad::new(FLOWERS_FRAGMENT_SHADER, VERTEX_SHADER)?;
    let mut blobby_shader = programs::Quad::new(SOLID_FRAGMENT_SHADER, VERTEX_SHADER)?;
    //let mut snake_shader = programs::Quad::new(SNAKE_FRAGMENT_SHADER, VERTEX_SHADER)?;

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
            if current_frame < FRAMES_PER_SECOND * 16 {
                blobby_shader.update(current_frame);
                //} else if current_frame < FRAMES_PER_SECOND * 30 {
                //flower_shader.update(current_frame);
                //} else if current_frame < FRAMES_PER_SECOND * 48 {
                //blobby_shader.update(current_frame);
                //} else if current_frame < FRAMES_PER_SECOND * 60 {
                //snake_shader.update(current_frame);
                //} else {
                //return Ok(());
            }

            delta_time -= FRAME_LENGTH_DURATION;
            current_frame += 1;
        }

        if current_frame < FRAMES_PER_SECOND * 16 {
            blobby_shader.render(current_frame);
            //} else if current_frame < FRAMES_PER_SECOND * 30 {
            //flower_shader.render(current_frame);
            //} else if current_frame < FRAMES_PER_SECOND * 48 {
            //blobby_shader.render(current_frame);
            //} else if current_frame < FRAMES_PER_SECOND * 60 {
            //snake_shader.render(current_frame);
            //} else {
            //return Ok(());
        }

        unsafe { glx::glXSwapBuffers(display as *mut bindings::glx::_XDisplay, window) }

        let events_pending = unsafe { Xlib::XPending(display) };
        for _ in 0..events_pending {
            let event = unsafe {
                let mut event: mem::MaybeUninit<Xlib::XEvent> = mem::MaybeUninit::uninit();
                Xlib::XNextEvent(display, event.as_mut_ptr());
                event.assume_init()
            };

            println!("event.type = %d\n\0", event.type_);
            match unsafe { event.type_ } {
                Xlib_constants::Expose => {
                    println!("Window attributes!\n\0");
                    unsafe {
                        let window_attributes = {
                            let mut window_attributes: mem::MaybeUninit<Xlib::XWindowAttributes> =
                                mem::MaybeUninit::uninit();
                            Xlib::XGetWindowAttributes(
                                display,
                                window,
                                window_attributes.as_mut_ptr(),
                            );
                            window_attributes.assume_init()
                        };
                        gl::glViewport(0, 0, window_attributes.width, window_attributes.height)
                    };
                }
                // Xlib_constants::ClientMessage => {
                //     println!("ClientMessage\n\0");
                //     let xclient = unsafe { event.xclient };
                //     if xclient.message_type == wm_protocols_atom && xclient.format == 32 {
                //         let protocol = unsafe { xclient.data.l }.as_ref()[0] as Xlib::Atom;
                //         if protocol == wm_delete_window_atom {
                //             return Ok(());
                //         }
                //     }
                //     println!("Received event type of %d\n\0", xclient.message_type);
                // }
                Xlib_constants::KeyPress => {
                    println!("Keyboard was pressed %d\n\0", event.xkey.keycode);
                    if unsafe { event.xkey }.keycode == 66 {
                        return Ok(());
                    }
                }
                _ => (),
            }
        }
    }
}
