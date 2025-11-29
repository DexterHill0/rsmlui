#[cxx::bridge(namespace = "rsmlui")]
mod ffi {

    unsafe extern "C++" {
        include!("rsmlui/Core.h");

        type SystemInterface = crate::ffi::backend::SystemInterface;
        type RenderInterface = crate::ffi::backend::RenderInterface;

        type Context = crate::context::Context;

        fn get_version() -> String;

        fn initialise() -> bool;

        fn create_context(name: String, width: i32, height: i32) -> *mut Context;

        unsafe fn set_system_interface(system_interface: *mut SystemInterface);
        unsafe fn set_render_interface(render_interface: *mut RenderInterface);

    }
}

pub use ffi::*;
