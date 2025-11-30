use crate::{core::element_document::ElementDocument, errors::RsmlUiError};

pub struct Context {
    pub(crate) raw: *mut rsmlui_sys::context::Context,
}

impl Context {
    pub fn update(&mut self) -> Result<(), RsmlUiError> {
        if !unsafe { rsmlui_sys::context::context_update(self.raw) } {
            return Err(RsmlUiError::ContextUpdateFailed);
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), RsmlUiError> {
        if !unsafe { rsmlui_sys::context::context_render(self.raw) } {
            return Err(RsmlUiError::ContextRenderFailed);
        }

        Ok(())
    }

    pub fn load_document<P: Into<String>>(&mut self, document_path: P) -> Option<ElementDocument> {
        let raw =
            unsafe { rsmlui_sys::context::context_load_document(self.raw, document_path.into()) };

        if raw.is_null() {
            return None;
        }

        Some(ElementDocument { raw })
    }
}
