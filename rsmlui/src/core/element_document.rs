use drop_tree::drop_tree;

use crate::utils::raw::{Ptr, Raw};

#[drop_tree(borrows(crate::core::context::Context))]
pub struct ElementDocument {
    pub(crate) raw: Ptr<ElementDocument>,
}

impl Raw for ElementDocument {
    type Ptr = *mut rsmlui_sys::element_document::ElementDocument;

    fn raw(&self) -> Self::Ptr {
        self.raw
    }
}

impl ElementDocument {
    pub fn show(&self) {
        unsafe { rsmlui_sys::element_document::element_document_show(self.raw) }
    }
}
