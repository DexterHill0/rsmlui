#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        type SystemInterface;

        type Vector2f = crate::Rml_Vector2f;
    }

    #[namespace = "Rml::Log"]
    extern "C++" {
        type Type = crate::Rml_Log_Type;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        unsafe fn system_interface_get_elapsed_time(system_interface: *mut SystemInterface) -> f64;

        unsafe fn system_interface_translate_string(
            system_interface: *mut SystemInterface,
            translated: Pin<&mut CxxString>,
            input: &CxxString,
        ) -> i32;

        unsafe fn system_interface_join_path(
            system_interface: *mut SystemInterface,
            translated_path: Pin<&mut CxxString>,
            document_path: &CxxString,
            path: &CxxString,
        );

        unsafe fn system_interface_log_message(
            system_interface: *mut SystemInterface,
            log_level: Type,
            message: &CxxString,
        ) -> bool;

        unsafe fn system_interface_set_mouse_cursor(
            system_interface: *mut SystemInterface,
            cursor_name: &CxxString,
        );

        unsafe fn system_interface_set_clipboard_text(
            system_interface: *mut SystemInterface,
            text: &CxxString,
        );

        unsafe fn system_interface_get_clipboard_text(
            system_interface: *mut SystemInterface,
            text: Pin<&mut CxxString>,
        );

        unsafe fn system_interface_activate_keyboard(
            system_interface: *mut SystemInterface,
            caret_position: Vector2f,
            line_height: f32,
        );

        unsafe fn system_interface_deactivate_keyboard(system_interface: *mut SystemInterface);
    }
}

pub use ffi::*;
