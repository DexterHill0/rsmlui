use std::rc::Rc;

use crate::core::core::AppOwner;
use crate::core::element_document::ElementDocument;
use crate::errors::RsmlUiError;
use crate::utils::raw::{Ptr, Raw};

#[repr(transparent)]
pub(crate) struct ContextOwner(Ptr<Context>);

pub struct Context {
    pub(crate) raw: Rc<ContextOwner>,
    pub(crate) _parent: Rc<AppOwner>,
}

impl Raw for Context {
    type Ptr = *mut rsmlui_sys::context::Context;

    fn raw(&self) -> Self::Ptr {
        self.raw.0
    }
}

impl Context {
    pub(crate) fn from_raw(ptr: Ptr<Self>, parent: &Rc<AppOwner>) -> Self {
        Self {
            raw: Rc::new(ContextOwner(ptr)),
            _parent: Rc::clone(parent),
        }
    }

    pub fn update(&self) -> Result<(), RsmlUiError> {
        if !unsafe { rsmlui_sys::context::context_update(self.raw.0) } {
            return Err(RsmlUiError::ContextUpdateFailed);
        }

        Ok(())
    }

    pub fn render(&self) -> Result<(), RsmlUiError> {
        if !unsafe { rsmlui_sys::context::context_render(self.raw.0) } {
            return Err(RsmlUiError::ContextRenderFailed);
        }

        Ok(())
    }

    pub fn load_document<P: Into<String>>(
        &self,
        document_path: P,
    ) -> Result<ElementDocument, RsmlUiError> {
        let raw =
            unsafe { rsmlui_sys::context::context_load_document(self.raw.0, document_path.into()) };

        if raw.is_null() {
            return Err(RsmlUiError::DocumentCreateFailed);
        }

        Ok(ElementDocument::from_raw(raw, &self.raw))
    }
}
