#[cxx::bridge(namespace = "rsmlui")]
mod ffi {

    unsafe extern "C++" {
        include!("rsmlui/Context.h");

        type Context;
        type ElementDocument = crate::element_document::ElementDocument;

        unsafe fn context_update(ctx: *mut Context) -> bool;
        unsafe fn context_render(ctx: *mut Context) -> bool;
        unsafe fn context_load_document(
            ctx: *mut Context,
            document_path: String,
        ) -> *mut ElementDocument;
    }
}

pub use ffi::*;
