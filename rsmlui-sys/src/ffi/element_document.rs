#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    unsafe extern "C++" {
        type ElementDocument;

        type ModalFlag = crate::Rml_ModalFlag;
        type FocusFlag = crate::Rml_FocusFlag;
        type ScrollFlag = crate::Rml_ScrollFlag;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/ElementDocument.h");

        unsafe fn element_document_show(
            ctx: *mut ElementDocument,
            modal_flag: ModalFlag,
            focus_flag: FocusFlag,
            scroll_flag: ScrollFlag,
        );
    }
}

pub use ffi::*;
