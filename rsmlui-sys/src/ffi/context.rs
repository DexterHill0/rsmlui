#[cxx::bridge(namespace = "rsmlui")]
mod ffi {

    #[namespace = "Rml"]
    unsafe extern "C++" {
        type Context;
        type ElementDocument = crate::element_document::ElementDocument;

        type Vector2i = crate::Rml_Vector2i;
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
    }
}

pub use ffi::*;
