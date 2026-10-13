use bon::Builder;
use drop_tree::drop_tree;
use rsmlui_macros::rmldoc;

use crate::_private::not_send_sync;
use crate::object::constness::Mut;
use crate::object::foreign::{ForeignHandle, ForeignObject};
use crate::object::ownership::Foreign;
use crate::types::document::{FocusFlag, ModalFlag, ScrollFlag};
use crate::utils::conversions::IntoSys;

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

pub mod tags {
    pub enum TagElementDocument {}
}

impl ForeignObject for tags::TagElementDocument {
    type Object = rsmlui_sys::element_document::ElementDocument;
}

pub type ElementDocumentHandle = ForeignHandle<'static, tags::TagElementDocument, Foreign, Mut>;

#[rmldoc(file = "api_Rml-ElementDocument.md", name = "Rml::ElementDocument")]
#[drop_tree(borrows(crate::core::context::Context))]
pub struct ElementDocument {
    handle: ElementDocumentHandle,
}

not_send_sync!(ElementDocument);

#[rmldoc(file = "api_Rml-ElementDocument.md")]
impl ElementDocument {
    /// See [`ElementDocument::show_with_options`] for options.
    #[rmldoc(name = "Rml::ElementDocument::Show")]
    #[inline]
    pub fn show(&mut self) {
        self.show_with_options(ShowOptions::default());
    }

    #[rmldoc(name = "Rml::ElementDocument::Show")]
    pub fn show_with_options(&mut self, options: ShowOptions) {
        unsafe {
            rsmlui_sys::element_document::element_document_show(
                self.handle.as_mut_ptr(),
                options.modal_flag.into_sys(),
                options.focus_flag.into_sys(),
                options.scroll_flag.into_sys(),
            )
        }
    }
}
