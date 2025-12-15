#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    unsafe extern "C++" {
        type ElementDocument;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/ElementDocument.h");

        unsafe fn element_document_destructor(ctx: *mut ElementDocument);

        unsafe fn element_document_show(ctx: *mut ElementDocument);
    }
}

pub use ffi::*;
