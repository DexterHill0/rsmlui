use cxx::{ExternType, type_id};

unsafe impl ExternType for crate::Rml_ModalFlag {
    type Id = type_id!("Rml::ModalFlag");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_FocusFlag {
    type Id = type_id!("Rml::FocusFlag");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_ScrollFlag {
    type Id = type_id!("Rml::ScrollFlag");
    type Kind = cxx::kind::Trivial;
}

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
