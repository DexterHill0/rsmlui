#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "SystemInterface"]
        type RmlSystemInterface = crate::system_interface::RmlSystemInterface;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/InterfaceDecls.h");

        type Opaque = crate::interfaces::Opaque;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        unsafe fn new_glfw_system_interface(glfw_window: *mut Opaque) -> *mut RmlSystemInterface;

        /// # Safety
        ///
        /// - `interface` must be a valid, non-null pointer to a `SystemInterface_Glfw`.
        unsafe fn glfw_system_interface_destructor(interface: *mut RmlSystemInterface);
    }
}

pub use ffi::{glfw_system_interface_destructor, new_glfw_system_interface};
