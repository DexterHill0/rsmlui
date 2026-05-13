use drop_tree::drop_tree;
use rsmlui_macros::rmldoc;

use crate::core::element_document::ElementDocument;
use crate::errors::Error;
use crate::not_send_sync;
use crate::utils::raw::{Ptr, Raw};

#[rmldoc(file = "api_Rml-Context.md", name = "Rml::Context")]
#[drop_tree(borrows(crate::core::core::Rml))]
pub struct Context {
    pub(crate) raw: Ptr<Context>,
}

not_send_sync!(Context);

impl Raw for Context {
    type Ptr = *mut rsmlui_sys::context::Context;

    #[inline(always)]
    fn raw(&self) -> Self::Ptr {
        self.raw
    }
}

#[rmldoc(file = "api_Rml-Context.md")]
impl Context {
    #[rmldoc(name = "Rml::Context::Update")]
    pub fn update(&self) -> Result<(), Error> {
        if !unsafe { rsmlui_sys::context::context_update(self.raw) } {
            return Err(Error::ContextUpdateFailed);
        }

        Ok(())
    }

    #[rmldoc(name = "Rml::Context::Render")]
    pub fn render(&self) -> Result<(), Error> {
        if !unsafe { rsmlui_sys::context::context_render(self.raw) } {
            return Err(Error::ContextRenderFailed);
        }

        Ok(())
    }

    /// Returns the raw C++ context pointer.
    ///
    /// # Safety
    ///
    /// The pointer is valid for the lifetime of this `Context`. The caller must not
    /// store it beyond that lifetime, and must not use it after `Context` is dropped.
    #[inline(always)]
    pub unsafe fn as_raw_ptr(&self) -> *mut rsmlui_sys::context::Context {
        self.raw()
    }

    /// The document returned from this context belongs to the context, and it will keep the context alive for as long
    /// as the document lives to prevent unsoundness.
    #[rmldoc(
        name = "Rml::Context::LoadDocument",
        refid = "class_rml_1_1_context_1a2ba6bc9ce08f1ba834de53a207af79a3"
    )]
    pub fn load_document<P: Into<String>>(
        &self,
        document_path: P,
    ) -> Result<ElementDocument, Error> {
        let raw =
            unsafe { rsmlui_sys::context::context_load_document(self.raw, document_path.into()) };

        if raw.is_null() {
            return Err(Error::DocumentCreateFailed);
        }

        Ok(ElementDocument::new_with_borrow(raw, self))
    }
}
