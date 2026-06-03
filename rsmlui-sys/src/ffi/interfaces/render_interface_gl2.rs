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

        unsafe fn gl2_render_interface_set_viewport(
            interface: *mut RmlRenderInterface,
            width: i32,
            height: i32,
        );
        unsafe fn gl2_render_interface_begin_frame(interface: *mut RmlRenderInterface);
        unsafe fn gl2_render_interface_end_frame(interface: *mut RmlRenderInterface);
        unsafe fn gl2_render_interface_clear(interface: *mut RmlRenderInterface);
    }
}

pub use ffi::{
    gl2_render_interface_begin_frame, gl2_render_interface_clear, gl2_render_interface_destructor,
    gl2_render_interface_end_frame, gl2_render_interface_set_viewport, new_gl2_render_interface,
};
