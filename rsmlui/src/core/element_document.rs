pub struct ElementDocument {
    pub(crate) raw: *mut rsmlui_sys::element_document::ElementDocument,
}

impl ElementDocument {
    pub fn show(&self) {
        unsafe { rsmlui_sys::element_document::element_document_show(self.raw) }
    }
}
