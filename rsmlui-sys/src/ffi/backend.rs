use cxx::{type_id, ExternType};

unsafe impl ExternType for crate::Rml_Input_KeyIdentifier {
    type Id = type_id!("Rml::Input::KeyIdentifier");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod ffi {

    #[namespace = "Rml::Input"]
    extern "C++" {
        type KeyIdentifier = crate::Rml_Input_KeyIdentifier;
    }

    #[namespace = "Rml"]

    extern "C++" {
        type Context = crate::context::Context;
        type SystemInterface = crate::system_interface::SystemInterface;
        type RenderInterface = crate::render_interface::RenderInterface;
    }

    #[namespace = "rsmlui::backend"]
    unsafe extern "C++" {
        include!("rsmlui/Backend.h");

        fn initialize(window_name: String, width: i32, height: i32, allow_resize: bool) -> bool;
        fn shutdown();

        fn get_system_interface() -> *mut SystemInterface;
        fn get_render_interface() -> *mut RenderInterface;

        unsafe fn process_events(
            context: *mut Context,
            key_down_callback: unsafe fn(
                context: *mut Context,
                key: KeyIdentifier,
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
}

pub use ffi::*;
