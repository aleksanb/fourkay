use crate::bindings::Xlib;
use core::mem;
use core::ptr;

#[macro_use]
pub mod println;
pub mod gl_utils;
pub mod gl_wrapper;

pub fn sleep(milliseconds: libc::c_long) {
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

pub fn xlib_events_ready(display: *mut Xlib::Display) -> i32 {
    unsafe {
        let x11_fd = Xlib::XConnectionNumber(display);
        let mut in_fds = mem::MaybeUninit::uninit();
        libc::FD_ZERO(in_fds.as_mut_ptr());
        libc::FD_SET(x11_fd, in_fds.as_mut_ptr());
        let mut select_timeout = libc::timeval {
            tv_sec: 0,
            tv_usec: 2_000,
        };
        libc::select(
            x11_fd + 1,
            in_fds.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut select_timeout,
        )
    }
}

pub mod time {
    use core::mem;
    use core::time::Duration;

    pub fn now() -> libc::timespec {
        let mut time = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        update(&mut time);
        time
    }

    pub fn update(time: &mut libc::timespec) {
        unsafe {
            libc::clock_gettime(libc::CLOCK_REALTIME, time);
        }
    }

    pub fn subtract(left: &libc::timespec, right: &libc::timespec) -> Duration {
        Duration::new(left.tv_sec as u64, left.tv_nsec as u32)
            - Duration::new(right.tv_sec as u64, right.tv_nsec as u32)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_time() {
            let time = now();
            assert_ne!(time.tv_sec, 0);

            let time2 = now();
            let delta = subtract(&time2, &time);
            assert!(delta > Duration::from_secs(0));
        }
    }
}
