use bon::Builder;
use drop_tree::drop_tree;
use rsmlui_macros::rmldoc;

use crate::not_send_sync;
use crate::types::flags::{FocusFlag, ModalFlag, ScrollFlag};
use crate::utils::raw::{Ptr, Raw};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Builder)]
#[builder(const)]
pub struct ShowOptions {
    #[builder(default = ModalFlag::None)]
    modal_flag: ModalFlag,
    #[builder(default = FocusFlag::Auto)]
    focus_flag: FocusFlag,
    #[builder(default = ScrollFlag::Auto)]
    scroll_flag: ScrollFlag,
}

impl Default for ShowOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

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
    /// See [`ElementDocument::show_with_options`] for options.
    #[rmldoc(name = "Rml::ElementDocument::Show")]
    #[inline]
    pub fn show(&self) {
        self.show_with_options(ShowOptions::default());
    }

    #[rmldoc(name = "Rml::ElementDocument::Show")]
    pub fn show_with_options(&self, options: ShowOptions) {
        unsafe {
            rsmlui_sys::element_document::element_document_show(
                self.raw,
                options.modal_flag,
                options.focus_flag,
                options.scroll_flag,
            )
        }
    }
}
