#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    unsafe extern "C++" {
        type SystemInterface = crate::ffi::system_interface::SystemInterface;
        type RenderInterface = crate::ffi::render_interface::RenderInterface;

        type Context = crate::context::Context;

        type Vector2i = crate::Rml_Vector2i;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Core.h");

        fn get_version() -> String;

        fn initialise() -> bool;
        fn shutdown();

        fn create_context(name: String, dimensions: Vector2i) -> *mut Context;

        unsafe fn set_system_interface(system_interface: *mut SystemInterface);
        unsafe fn set_render_interface(render_interface: *mut RenderInterface);

    }
}

pub use ffi::*;
