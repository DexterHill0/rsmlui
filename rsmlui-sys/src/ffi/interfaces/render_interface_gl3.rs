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

        fn new_gl3_render_interface() -> *mut RmlRenderInterface;

        /// # Safety
        ///
        /// - `interface` must be a valid, non-null pointer to a `RenderInterface_GL3`.
        unsafe fn gl3_render_interface_destructor(interface: *mut RmlRenderInterface);
    }
}

pub use ffi::{gl3_render_interface_destructor, new_gl3_render_interface};
