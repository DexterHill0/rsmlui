#[cxx::bridge(namespace = "rsmlui")]
mod ffi {
    unsafe extern "C++" {
        include!("rsmlui/Backend.h");

        type Context = crate::context::Context;
        type SystemInterface;
        type RenderInterface;

        fn initialize(window_name: String, width: i32, height: i32, allow_resize: bool) -> bool;
        fn shutdown();

        fn get_system_interface() -> *mut SystemInterface;
        fn get_render_interface() -> *mut RenderInterface;

        unsafe fn process_events(
            context: *mut Context,
            key_down_callback: unsafe fn(
                context: *mut Context,
                key: i32,
                key_modifier: i32,
                native_dp_ratio: f32,
                priority: bool,
            ) -> bool,
            power_save: bool,
        ) -> bool;

        fn request_exit();

        fn begin_frame();
        fn present_frame();
    }

    unsafe extern "C++" {
        type KeyDownCallback;
    }

    // extern "Rust" {
    //     fn my_key_down_callback(
    //         context: *mut Context,
    //         key: i32, // TODO Rml::Input::KeyIdentifier enum?
    //         key_modifier: i32,
    //         native_dp_ratio: f32,
    //         priority: bool,
    //     ) -> bool;
    // }
}

pub use ffi::*;
