#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "SystemInterface"]
        type RmlSystemInterface = crate::system_interface::RmlSystemInterface;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        fn new_win32_system_interface() -> *mut RmlSystemInterface;

        /// # Safety
        ///
        /// - `interface` must be a valid, non-null pointer to a `SystemInterface_Win32`.
        unsafe fn win32_system_interface_destructor(interface: *mut RmlSystemInterface);
    }
}

pub use ffi::{new_win32_system_interface, win32_system_interface_destructor};
