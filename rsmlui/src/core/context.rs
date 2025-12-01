use std::marker::PhantomData;

use crate::core::core::RsmlUi;
use crate::core::element_document::ElementDocument;
use crate::errors::RsmlUiError;
use crate::interfaces::backend::Backend;

pub struct Context<'ctx> {
    pub(crate) raw: *mut rsmlui_sys::context::Context,
    pub(crate) _phantom: PhantomData<&'ctx ()>,
}

impl<'ctx> Context<'ctx> {
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

    pub fn load_document<'doc: 'ctx, P: Into<String>>(
        &self,
        document_path: P,
    ) -> Option<ElementDocument<'doc>> {
        let raw =
            unsafe { rsmlui_sys::context::context_load_document(self.raw, document_path.into()) };

        if raw.is_null() {
            return None;
        }

        Some(ElementDocument {
            raw,
            _phantom: PhantomData,
        })
    }
}
