#[cxx::bridge]
mod ffi {
    #[namespace = "rsmlui::rust_system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        #[doc(hidden)]
        pub(super) type RustSystemInterface;
    }

    impl UniquePtr<RustSystemInterface> {}
}

pub use ffi::*;
