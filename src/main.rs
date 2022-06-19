#![feature(lang_items, start, raw_ref_op, stmt_expr_attributes)]
#![no_std]
#![no_main]
// https://doc.rust-lang.org/1.19.0/reference/attributes.html#crate-only-attributes
#![allow(warnings)]

#[macro_use]
mod shitty;
use crate::bindings::alsa::{self, snd_pcm_hw_params_any};
use crate::bindings::gl::GL_SAMPLE_BUFFERS;
use crate::bindings::math;

use self::shitty::{gl_wrapper, println::*};

use self::programs::Program;
use core::mem::{self, MaybeUninit};
use core::ptr;

mod bindings;
mod programs;

use self::bindings::{gl, glx, Xlib, Xlib_constants};

#[cfg(not(test))]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    #[cfg(not(feature = "error-handling"))]
    {
        use core::hint::unreachable_unchecked;
        unsafe {
            unreachable_unchecked();
        }
    }
    #[cfg(feature = "error-handling")]
    {
        loop {}
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

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

static VERTEX_SHADER: &str = concat!(include_str!("shaders/quad-vertex.glsl"), "\0");
// static BALLS_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/balls.glsl.out"), "\0");
// static SOLID_FRAGMENT_SHADER: &str = "void main(){gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);}\0";
// static FLOWERS_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/flower.glsl"), "\0");
// static BLOBBY_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/blobby.glsl.out"), "\0");
// static SNAKE_FRAGMENT_SHADER: &str = concat!(include_str!("shaders/snake.glsl.out"), "\0");
// static RAYMARCHER: &str = concat!(include_str!("shaders/raymarcher-fragment.glsl"), "\0");

// 2022 shaders
// blobs
// static VORONOI_SHADER: &str = concat!(include_str!("shaders/2022/voronoi.glsl"), "\0");
// static DISCOLINES_SHADER: &str = concat!(include_str!("shaders/2022/discolines.glsl"), "\0");
static BLOBS_SHADER: &str = concat!(include_str!("shaders/2022/blobs.glsl.out"), "\0");

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let mut pcm_handle: *mut alsa::snd_pcm_t = MaybeUninit::uninit().assume_init();
        let res = alsa::snd_pcm_open(
            &mut pcm_handle,
            "default\0".as_ptr() as *const _,
            alsa::SND_PCM_STREAM_PLAYBACK,
            0,
        );
        if res < 0 {
            println!("1Res: %s\n\0", alsa::snd_strerror(res));
        }

        let mut params: *mut alsa::snd_pcm_hw_params_t = MaybeUninit::uninit().assume_init();
        let res = alsa::snd_pcm_hw_params_malloc(&mut params);
        #[cfg(feature = "error-handling")]
        {
            if res < 0 {
                println!("2Res: %s\n\0", alsa::snd_strerror(res));
            }
        }

        let res = alsa::snd_pcm_hw_params_any(pcm_handle, params);
        if res < 0 {
            println!("3Res: %s\n\0", alsa::snd_strerror(res));
        }

        let res = alsa::snd_pcm_hw_params_set_access(
            pcm_handle,
            params,
            alsa::SND_PCM_ACCESS_RW_INTERLEAVED,
        );
        if res < 0 {
            println!("4Res: %s\n\0", alsa::snd_strerror(res));
        }

        let res =
            alsa::snd_pcm_hw_params_set_format(pcm_handle, params, alsa::SND_PCM_FORMAT_S16_LE);
        if res < 0 {
            println!("5Res: %s\n\0", alsa::snd_strerror(res));
        }

        const channels: u32 = 1;
        let res = alsa::snd_pcm_hw_params_set_channels(pcm_handle, params, channels);
        if res < 0 {
            println!("6Res: %s\n\0", alsa::snd_strerror(res));
        }

        let mut rate: libc::c_uint = 44100;
        let res = alsa::snd_pcm_hw_params_set_rate_near(
            pcm_handle,
            params,
            &mut rate as *mut _,
            &mut 0 as *mut _,
        );
        #[cfg(feature = "error-handling")]
        {
            if res < 0 {
                println!("7Res: %s\n\0", alsa::snd_strerror(res));
            }
        }
        println!("Rate chosen: %d\n\0", rate);

        let res = alsa::snd_pcm_hw_params(pcm_handle, params);
        #[cfg(feature = "error-handling")]
        {
            if res < 0 {
                println!("8Res: %s\n\0", alsa::snd_strerror(res));
            }
        }

        #[cfg(feature = "error-handling")]
        {
            println!("PCM name: %s\n\0", alsa::snd_pcm_name(pcm_handle));
            println!(
                "PCM state: %s\n\0",
                alsa::snd_pcm_state_name(alsa::snd_pcm_state(pcm_handle))
            );

            let mut tmp: libc::c_uint = 0;
            alsa::snd_pcm_hw_params_get_channels(params, &mut tmp as *mut _);
            println!("Channels: %d\n\0", tmp);

            alsa::snd_pcm_hw_params_get_rate(params, &mut tmp as *mut _, &mut 0 as *mut _);
            println!("Rate: %d\n\0", tmp);
        }
        let mut frames: alsa::snd_pcm_uframes_t = MaybeUninit::uninit().assume_init();
        alsa::snd_pcm_hw_params_get_period_size(params, &mut frames, &mut 0);
        println!("Frames: %d\n\0", frames);

        #[cfg(feature = "error-handling")]
        {
            let mut period_time = 0;
            let res =
                alsa::snd_pcm_hw_params_get_period_time(params, &mut period_time, ptr::null_mut());
            if res < 0 {
                println!("10Res: %s\n\0", alsa::snd_strerror(res));
            }
            println!("Period time: %d\n\0", period_time);
        }

        // Now we set up the graphics stuffs
        let display = Xlib::XOpenDisplay(ptr::null());
        #[cfg(feature = "error-handling")]
        {
            if display.is_null() {
                println!("Couldn't set up display\n\0");
                return 1;
            }
        }

        let glx_display = display as *mut glx::Display;
        #[cfg(feature = "error-handling")]
        {
            let mut glx_major = 0;
            let mut glx_minor = 0;
            let glx_result = glx::glXQueryVersion(glx_display, &mut glx_major, &mut glx_minor);
            println!(
                "glX version: Major: %d, minor: %d, result: %d\n\0",
                glx_major, glx_minor, glx_result
            );
        }

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

        //let window = Xlib::XCreateSimpleWindow(display, root_window, 0, 0, 1920, 1080, 0, 0, 0);
        //println!("Window: %lu\n\0", window);

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

            let gl_renderer = gl_wrapper::glGetString(gl::GL_RENDERER);
            println!("Renderer: %s\n\0", gl_renderer as *const libc::c_char);

            let gl_vendor = gl_wrapper::glGetString(gl::GL_VENDOR);
            println!("Vendor: %s\n\0", gl_vendor as *const libc::c_char);
        }

        const C4: f32 = 261.63;
        const Cs4: f32 = 277.18;
        const D4: f32 = 293.66;
        const Ds4: f32 = 311.13;
        const E4: f32 = 329.63;
        const F4: f32 = 349.23;
        const Fs4: f32 = 369.99;
        const G4: f32 = 392.00;
        const Gs4: f32 = 415.30;
        const A4: f32 = 440.00;
        const As4: f32 = 466.16;
        const B4: f32 = 493.88;

        //let mut notes = [0; 40];
        //for i in 0..28 {
        //let pow = (20f32 - i as f32) / 12;
        //notes[i] = math::pow(440f32, pow);
        ////notes[0] = math:: 440f32 *
        //}

        let sample_rate = 44_100;
        const BPM: usize = 170 * 4;
        let note_length_in_samples = sample_rate * 60 / BPM;

        let note_frequencies: [f32; 12] = [C4, Cs4, D4, Ds4, E4, F4, Fs4, G4, Gs4, A4, As4, B4];
        let note_frequencies: [f32; 5] = [C4, Ds4, F4, G4, As4];

        //let note_frequencies = notes;
        let notes_length: usize = note_frequencies.len() as usize;

        // We use [0, 1) as volume internally, and then we normalize when converting to i8.
        let play_note = |frequency: f32, sample_idx: usize| -> f32 {
            let wavelength_in_samples = sample_rate as f32 / frequency;
            let how_far_into_note =
                ((sample_idx as f32) % wavelength_in_samples) / wavelength_in_samples;

            how_far_into_note
        };

        let get_amplitude_for_sample_index = |sample_idx: usize| -> f32 {
            let note_idx = sample_idx / note_length_in_samples;
            let frequency = note_frequencies[(note_idx % notes_length) as usize];
            // println!("\nsample_idx %d\n\0", sample_idx as libc::c_uint);
            // println!("nidx %d\n\0", note_idx as libc::c_uint);
            // println!("frec %f\n\0", frequency as libc::c_double);

            let mut sum = 0f32;
            sum += play_note(frequency * 2f32, sample_idx);
            sum += play_note(frequency, sample_idx);
            sum += play_note(frequency / 2f32, sample_idx);
            sum /= 3f32;
            sum
        };

        let samples_to_prerender = 8 * 60 * sample_rate;
        let bytes_per_sample = mem::size_of::<i16>() * 1; // Because 16bit audio
        let buffer_size = bytes_per_sample * samples_to_prerender;
        let mut buffer = libc::malloc(buffer_size as libc::size_t) as *mut i16;
        println!("Allocating buffer with : %d\n\0", buffer_size);

        for sample_idx in 0..samples_to_prerender {
            let sample = get_amplitude_for_sample_index(sample_idx);

            let rendered_sample = (sample * (u16::MAX - 1) as f32 - (u16::MAX / 2) as f32) as i16;
            *buffer.offset(sample_idx as isize) = rendered_sample;
        }

        const FRAMES_PER_SECOND: usize = 60;
        const FRAME_LENGTH_MILLISECONDS: usize = 1_000 / FRAMES_PER_SECOND;
        const FRAME_LENGTH_DURATION: core::time::Duration =
            core::time::Duration::from_millis(FRAME_LENGTH_MILLISECONDS as _);
        let mut current_frame = 0f32;
        let mut current_time = shitty::time::now();
        let mut previous_time = shitty::time::now();
        let mut delta_time = core::time::Duration::new(0, 0);

        let mut solid_shader = programs::Quad::new(BLOBS_SHADER, VERTEX_SHADER).unwrap();
        // VORONOI_SHADER
        // DISCOLINES_SHADER
        // BLOBS_SHADER

        // let mut timestamp: alsa::snd_timestamp_t = MaybeUninit::uninit().assume_init();
        // let mut status: alsa::snd_pcm_status_t = MaybeUninit::uninit().assume_init();
        let mut current_sample_idx: isize = 0;
        loop {
            shitty::time::update(&mut current_time);
            let delta_since_last_wake = shitty::time::subtract(&current_time, &previous_time);
            delta_time += delta_since_last_wake;
            previous_time = current_time;

            while delta_time >= FRAME_LENGTH_DURATION {
                // Removed for size
                //solid_shader.update(current_frame as f32 / FRAMES_PER_SECOND as f32);

                delta_time -= FRAME_LENGTH_DURATION;
                current_frame += 1.0;

                // Sound can starve here?

                // alsa::snd_pcm_status_get_state(&mut status);
                // alsa::snd_pcm_status_get_tstamp(&status, &mut timestamp);
                // println!("sec %d\n\0", timestamp.tv_sec);
                //println!("usec %f\n\0", timestamp.tv_usec);

                let frames_to_write = alsa::snd_pcm_avail(pcm_handle);
                // println!("Frames to write, %d\n\0", frames_to_write);
                if frames_to_write > 0 {
                    //println!(
                    //"%d Writin frames, %d, but max %d\n\0",
                    //current_sample_idx, frames_to_write, fframes
                    //);
                    let frames_to_write = (frames_to_write as usize).min(frames as usize);
                    let res = alsa::snd_pcm_writei(
                        pcm_handle,
                        buffer.offset(current_sample_idx) as *const libc::c_void,
                        frames_to_write as _,
                    );
                    current_sample_idx += frames_to_write as isize;

                    #[cfg(feature = "error-handling")]
                    {
                        if res == -(libc::EPIPE as i64) {
                            alsa::snd_pcm_prepare(pcm_handle);
                            println!("Wrote %d \n\0", res);
                        } else if res < 0 {
                            println!("Fakk write failed: %d \n\0", res);
                        }
                    }
                }
            }

            // println!("rendeing frame %f\n\0", current_frame as libc::c_float);
            solid_shader.render(current_frame / FRAMES_PER_SECOND as f32);

            unsafe { glx::glXSwapBuffers(display as *mut bindings::glx::_XDisplay, window) }
            if should_exit_after_processing_pending_events(display, window) {
                return 1;
            }
        }

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

//fn main_loop(display: *mut Xlib::_XDisplay, window: Xlib::Window) -> Result<(), ()> {}

fn should_exit_after_processing_pending_events(
    display: *mut Xlib::_XDisplay,
    window: Xlib::Window,
) -> bool {
    let events_pending = unsafe { Xlib::XPending(display) };
    for _ in 0..events_pending {
        let event = unsafe {
            let mut event: mem::MaybeUninit<Xlib::XEvent> = mem::MaybeUninit::uninit();
            Xlib::XNextEvent(display, event.as_mut_ptr());
            event.assume_init()
        };

        println!("event.type = %d\n\0", event.type_.as_ref());
        match unsafe { event.type_.as_ref() } {
            &Xlib_constants::Expose => {
                println!("Window attributes!\n\0");
                unsafe {
                    let window_attributes = {
                        let mut window_attributes: mem::MaybeUninit<Xlib::XWindowAttributes> =
                            mem::MaybeUninit::uninit();
                        Xlib::XGetWindowAttributes(display, window, window_attributes.as_mut_ptr());
                        window_attributes.assume_init()
                    };
                    gl::glViewport(0, 0, window_attributes.width, window_attributes.height)
                };
            }
            &Xlib_constants::KeyPress => {
                println!("Keyboard was pressed %d\n\0", event.xkey.as_ref().keycode);
                // 9 is esc
                if unsafe { event.xkey.as_ref() }.keycode == 9 {
                    return true;
                }
            }
            _ => (),
        }
    }
    return false;
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
