#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "RenderInterface"]
        type RmlRenderInterface = crate::render_interface::RmlRenderInterface;
    }

    #[namespace = "rsmlui::render_interface"]
    unsafe extern "C++" {
        include!("rsmlui/RenderInterface.h");

        fn new_gl2_render_interface() -> *mut RmlRenderInterface;

        /// # Safety
        ///
        /// - `interface` must be a valid, non-null pointer to a `RenderInterface_GL2`.
        unsafe fn gl2_render_interface_destructor(interface: *mut RmlRenderInterface);
    }
}

pub use ffi::{gl2_render_interface_destructor, new_gl2_render_interface};
