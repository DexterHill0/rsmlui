#[macro_export]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        const _: [(); $left] = [(); $right];
    };
}

#[cxx::bridge]
mod ffi {
    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type c_void;
    }
}

pub use ffi::c_void;
