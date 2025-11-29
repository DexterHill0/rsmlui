#[cxx::bridge(namespace = "rsmlui")]
mod ffi {

    unsafe extern "C++" {
        include!("rsmlui/ElementDocument.h");

        type ElementDocument;

        unsafe fn element_document_show(ctx: *mut ElementDocument);
    }
}

pub use ffi::*;
