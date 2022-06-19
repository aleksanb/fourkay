pub trait Prepare {
    type Item;

    fn prepare(&self) -> Self::Item;
}

macro_rules! trivial_prepare {
    ($type:ty) => {
        impl Prepare for $type {
            type Item = $type;

            #[inline(always)]
            fn prepare(&self) -> Self::Item {
                *self
            }
        }
    };
}

impl Prepare for &'static str {
    type Item = *const libc::c_char;

    fn prepare(&self) -> Self::Item {
        self.as_ptr() as *const _
    }
}

trivial_prepare!(usize);
trivial_prepare!(isize);
trivial_prepare!(libc::c_float);
//trivial_prepare!(libc::c_double);
trivial_prepare!(libc::c_short);
trivial_prepare!(libc::c_int);
trivial_prepare!(libc::c_uint);
trivial_prepare!(libc::c_long);
trivial_prepare!(libc::c_ulong);
trivial_prepare!(u128);
trivial_prepare!(*const libc::c_char);
trivial_prepare!(*mut libc::c_char);

macro_rules! println {
    ($( $args:expr ),* ) => {
        #[cfg(feature = "println")]
        {
            unsafe {
                libc::printf(
                    $(
                       $args.prepare(),
                    )*
                );
            }
        }
    };
}
