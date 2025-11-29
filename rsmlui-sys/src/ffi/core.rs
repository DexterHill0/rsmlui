#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    unsafe extern "C++" {
        type SystemInterface = crate::ffi::backend::SystemInterface;
        type RenderInterface = crate::ffi::backend::RenderInterface;

        type Context = crate::context::Context;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Core.h");

        fn get_version() -> String;

        fn initialise() -> bool;
        fn shutdown();

        fn create_context(name: String, width: i32, height: i32) -> *mut Context;

        unsafe fn set_system_interface(system_interface: *mut SystemInterface);
        unsafe fn set_render_interface(render_interface: *mut RenderInterface);

    }
}

pub use ffi::*;
