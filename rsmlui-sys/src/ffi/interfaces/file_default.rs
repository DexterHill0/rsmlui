#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "FileInterface"]
        type RmlFileInterface = crate::file_interface::RmlFileInterface;
    }

    #[namespace = "rsmlui::file_interface"]
    unsafe extern "C++" {
        include!("rsmlui/FileInterface.h");

        fn new_default_file_interface() -> *mut RmlFileInterface;

        /// # Safety
        ///
        /// - `interface` must be a valid, non-null pointer to a `DefaultFileInterface`.
        unsafe fn default_file_interface_destructor(interface: *mut RmlFileInterface);
    }
}

pub use ffi::{default_file_interface_destructor, new_default_file_interface};
