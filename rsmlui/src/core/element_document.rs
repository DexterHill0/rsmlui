use std::marker::PhantomData;

pub struct ElementDocument<'doc> {
    pub(crate) raw: *mut rsmlui_sys::element_document::ElementDocument,
    pub(crate) _phantom: PhantomData<&'doc ()>,
}

impl<'doc> ElementDocument<'doc> {
    pub fn show(&self) {
        unsafe { rsmlui_sys::element_document::element_document_show(self.raw) }
    }
}
