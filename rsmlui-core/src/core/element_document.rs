use drop_tree::drop_tree;
use rsmlui_macros::rmldoc;

use crate::not_send_sync;
use crate::utils::raw::{Ptr, Raw};

#[rmldoc(file = "api_Rml-ElementDocument.md", name = "Rml::ElementDocument")]
#[drop_tree(borrows(crate::core::context::Context))]
pub struct ElementDocument {
    pub(crate) raw: Ptr<ElementDocument>,
}

not_send_sync!(ElementDocument);

impl Raw for ElementDocument {
    type Ptr = *mut rsmlui_sys::element_document::ElementDocument;

    fn raw(&self) -> Self::Ptr {
        self.raw
    }
}

#[rmldoc(file = "api_Rml-ElementDocument.md")]
impl ElementDocument {
    #[rmldoc(name = "Rml::ElementDocument::Show")]
    // TODO: missing arguments to function
    pub fn show(&self) {
        unsafe { rsmlui_sys::element_document::element_document_show(self.raw) }
    }
}
