#[cxx::bridge(namespace = "rsmlui")]
mod ffi {

    #[namespace = "Rml"]
    unsafe extern "C++" {
        type Context;
        type ElementDocument = crate::element_document::ElementDocument;

        type Vector2i = crate::Rml_Vector2i;
        type Vector2f = crate::Rml_Vector2f;
    }

    #[namespace = "Rml::Input"]
    extern "C++" {
        type KeyIdentifier = crate::Rml_Input_KeyIdentifier;
        type KeyModifier = crate::Rml_Input_KeyModifier;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Context.h");

        unsafe fn context_update(ctx: *mut Context) -> bool;
        unsafe fn context_render(ctx: *mut Context) -> bool;
        unsafe fn context_load_document(
            ctx: *mut Context,
            document_path: String,
        ) -> *mut ElementDocument;
        unsafe fn context_set_dimensions(ctx: *mut Context, dimensions: Vector2i);

        unsafe fn context_process_mouse_move(
            ctx: *mut Context,
            x: i32,
            y: i32,
            key_modifier_state: KeyModifier,
        ) -> bool;
        unsafe fn context_process_mouse_button_down(
            ctx: *mut Context,
            button_index: i32,
            key_modifier_state: KeyModifier,
        ) -> bool;
        unsafe fn context_process_mouse_button_up(
            ctx: *mut Context,
            button_index: i32,
            key_modifier_state: KeyModifier,
        ) -> bool;
        unsafe fn context_process_mouse_wheel(
            ctx: *mut Context,
            wheel_delta: Vector2f,
            key_modifier_state: KeyModifier,
        ) -> bool;
        unsafe fn context_process_mouse_leave(ctx: *mut Context) -> bool;
        unsafe fn context_process_key_down(
            ctx: *mut Context,
            key_identifier: KeyIdentifier,
            key_modifier_state: KeyModifier,
        ) -> bool;
        unsafe fn context_process_key_up(
            ctx: *mut Context,
            key_identifier: KeyIdentifier,
            key_modifier_state: KeyModifier,
        ) -> bool;
        unsafe fn context_process_text_input_char(ctx: *mut Context, character: i8) -> bool;
        unsafe fn context_process_text_input_string(ctx: *mut Context, string: &String) -> bool;
    }
}

pub use ffi::{
    Context, context_load_document, context_process_key_down, context_process_key_up,
    context_process_mouse_button_down, context_process_mouse_button_up,
    context_process_mouse_leave, context_process_mouse_move, context_process_mouse_wheel,
    context_process_text_input_char, context_process_text_input_string, context_render,
    context_set_dimensions, context_update,
};
