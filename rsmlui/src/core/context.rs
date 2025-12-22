use drop_tree::drop_tree;

use crate::core::element_document::ElementDocument;
use crate::errors::RsmlUiError;
use crate::not_send_sync;
use crate::utils::raw::{Ptr, Raw};

#[drop_tree(borrows(crate::core::core::RsmlUi))]
pub struct Context {
    pub(crate) raw: Ptr<Context>,
}

not_send_sync!(Context);

impl Raw for Context {
    type Ptr = *mut rsmlui_sys::context::Context;

    fn raw(&self) -> Self::Ptr {
        self.raw
    }
}

impl Context {
    pub fn update(&self) -> Result<(), RsmlUiError> {
        if !unsafe { rsmlui_sys::context::context_update(self.raw) } {
            return Err(RsmlUiError::ContextUpdateFailed);
        }

        Ok(())
    }

    pub fn render(&self) -> Result<(), RsmlUiError> {
        if !unsafe { rsmlui_sys::context::context_render(self.raw) } {
            return Err(RsmlUiError::ContextRenderFailed);
        }

        Ok(())
    }

    pub fn load_document<P: Into<String>>(
        &self,
        document_path: P,
    ) -> Result<ElementDocument, RsmlUiError> {
        let raw =
            unsafe { rsmlui_sys::context::context_load_document(self.raw, document_path.into()) };

        if raw.is_null() {
            return Err(RsmlUiError::DocumentCreateFailed);
        }

        Ok(ElementDocument::new_with_borrow(raw, self))
    }
}
