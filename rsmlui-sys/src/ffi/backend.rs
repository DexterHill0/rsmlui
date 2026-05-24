#[cxx::bridge]
mod ffi {
    #[namespace = "Rml::Input"]
    extern "C++" {
        type KeyIdentifier = crate::Rml_Input_KeyIdentifier;
        type KeyModifier = crate::Rml_Input_KeyModifier;
    }

    #[namespace = "Rml"]
    extern "C++" {
        type Context = crate::context::Context;
        #[cxx_name = "SystemInterface"]
        type RmlSystemInterface = crate::system_interface::RmlSystemInterface;
        #[cxx_name = "RenderInterface"]
        type RmlRenderInterface = crate::render_interface::RmlRenderInterface;

        type Vector2i = crate::Rml_Vector2i;
    }

    #[namespace = "rsmlui::backend"]
    unsafe extern "C++" {
        include!("rsmlui/Backend.h");

        fn initialize(window_name: String, dimensions: Vector2i, allow_resize: bool) -> bool;
        fn shutdown();

        fn get_system_interface() -> *mut RmlSystemInterface;
        fn get_render_interface() -> *mut RmlRenderInterface;

        unsafe fn process_events(
            context: *mut Context,
            key_down_callback: unsafe fn(
                context: *mut Context,
                key: KeyIdentifier,
                key_modifier: KeyModifier,
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

pub use ffi::{
    begin_frame, get_render_interface, get_system_interface, initialize, present_frame,
    process_events, request_exit, shutdown,
};
