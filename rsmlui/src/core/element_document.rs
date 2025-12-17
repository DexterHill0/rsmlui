use std::rc::Rc;

use crate::core::context::ContextOwner;
use crate::utils::raw::{Ptr, Raw};

#[repr(transparent)]
pub(crate) struct ElementDocumentOwner(Ptr<ElementDocument>);

pub struct ElementDocument {
    pub(crate) raw: Rc<ElementDocumentOwner>,
    pub(crate) _parent: Rc<ContextOwner>,
}

impl Raw for ElementDocument {
    type Ptr = *mut rsmlui_sys::element_document::ElementDocument;

    fn raw(&self) -> Self::Ptr {
        self.raw.0
    }
}

impl ElementDocument {
    pub(crate) fn from_raw(raw: Ptr<Self>, parent: &Rc<ContextOwner>) -> Self {
        Self {
            raw: Rc::new(ElementDocumentOwner(raw)),
            _parent: Rc::clone(parent),
        }
    }

    pub fn show(&self) {
        unsafe { rsmlui_sys::element_document::element_document_show(self.raw.0) }
    }
}
