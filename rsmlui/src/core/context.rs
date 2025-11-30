use std::path::Path;

use crate::core::element_document::ElementDocument;

pub struct Context {
    pub(crate) raw: *mut rsmlui_sys::context::Context,
}

impl Context {
    pub fn update(&mut self) -> Result<(), ()> {
        if !unsafe { rsmlui_sys::context::context_update(self.raw) } {
            return Err(());
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), ()> {
        if !unsafe { rsmlui_sys::context::context_update(self.raw) } {
            return Err(());
        }

        Ok(())
    }

    pub fn load_document<P: Into<String>>(&mut self, document_path: P) -> ElementDocument {
        let raw =
            unsafe { rsmlui_sys::context::context_load_document(self.raw, document_path.into()) };

        ElementDocument { raw }
    }
}
