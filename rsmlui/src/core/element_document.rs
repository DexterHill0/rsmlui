use std::rc::Rc;

use crate::core::context::ContextOwner;
use crate::utils::raw::{Ptr, Raw};

#[repr(transparent)]
pub(crate) struct ElementDocumentOwner(Ptr<ElementDocument>);

impl Drop for ElementDocumentOwner {
    fn drop(&mut self) {
        unsafe { rsmlui_sys::element_document::element_document_destructor(self.0) };
    }
}

pub struct ElementDocument {
    pub(crate) raw: Rc<ElementDocumentOwner>,
    pub(crate) _parent: Rc<ContextOwner>,
}

impl Raw for ElementDocument {
    type Ptr = *mut rsmlui_sys::element_document::ElementDocument;
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
