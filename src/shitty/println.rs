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

trivial_prepare!(libc::c_int);
trivial_prepare!(libc::c_ulong);
trivial_prepare!(libc::c_uint);
trivial_prepare!(*const libc::c_char);
trivial_prepare!(*mut libc::c_char);

macro_rules! println {
    ($( $args:expr ),* ) => {
        unsafe {
            libc::printf(
                $(
                   $args.prepare(),
                )*
            );
        }
    };
}

// macro_rules! println {
//     ($($val:expr),*) => {};
// }