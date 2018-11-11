use core::ptr;
use crate::shitty::println::*;

#[macro_use]
pub mod println;
pub mod gl_utils;
pub mod gl_wrapper;

pub fn sleep(milliseconds: i64) {
    let mut sleep_timeout = libc::timeval {
        tv_sec: 0,
        tv_usec: milliseconds * 1000,
    };
    unsafe {
        libc::select(
            1,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut sleep_timeout,
        );
    }
}

pub fn print_duration(duration: core::time::Duration) {
    println!(
        "Duration: Seconds: %ld. Nanoseconds %d\n\0",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
