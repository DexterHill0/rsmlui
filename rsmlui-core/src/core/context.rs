use drop_tree::drop_tree;
use rsmlui_macros::rmldoc;

use crate::_private::{IntoSys, not_send_sync};
use crate::core::element_document::{ElementDocument, ElementDocumentHandle};
use crate::error::Error;
use crate::math::UVec2;
use crate::object::constness::NonConst;
use crate::object::foreign::{ForeignHandle, ForeignObject, ObjectMutPtr};
use crate::object::ownership::Foreign;

pub mod tags {
    pub enum TagContext {}
}

impl ForeignObject for tags::TagContext {
    type Object = rsmlui_sys::context::Context;
}

pub type ContextHandle = ForeignHandle<'static, tags::TagContext, Foreign, NonConst>;

#[rmldoc(file = "api_Rml-Context.md", name = "Rml::Context")]
#[drop_tree(borrows(crate::core::rml::Rml))]
pub struct Context {
    // TODO: currently marked as Foreign as we have no drop, but should we remove contexts on Drop?
    handle: ContextHandle,
}

// Currently not quite the same as `ContextHandle` due to the non-static lifetime.
// TODO: figure out how to also implement methods on this
pub type ContextRefMut<'a> = ForeignHandle<'a, tags::TagContext, Foreign, NonConst>;

not_send_sync!(Context);

// TODO: Context is required to be a named struct for drop-tree, not a type alias like
// other instances of Handles. That means there will be issues with implementing
// methods on both Context and (future) ContextRef at the same time
#[rmldoc(file = "api_Rml-Context.md")]
impl Context {
    #[rmldoc(name = "Rml::Context::Update")]
    pub fn update(&mut self) -> Result<(), Error> {
        if !unsafe { rsmlui_sys::context::context_update(self.handle.as_mut_ptr()) } {
            return Err(Error::ContextUpdateFailed);
        }

        Ok(())
    }

    #[rmldoc(name = "Rml::Context::Render")]
    pub fn render(&mut self) -> Result<(), Error> {
        if !unsafe { rsmlui_sys::context::context_render(self.handle.as_mut_ptr()) } {
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
    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> ObjectMutPtr<tags::TagContext> {
        unsafe { self.handle.as_mut_ptr() }
    }

    /// The document returned from this context belongs to the context, and it will keep the context alive for as long
    /// as the document lives to prevent unsoundness.
    #[rmldoc(
        name = "Rml::Context::LoadDocument",
        refid = "class_rml_1_1_context_1a2ba6bc9ce08f1ba834de53a207af79a3"
    )]
    pub fn load_document<P: Into<String>>(
        &mut self,
        document_path: P,
    ) -> Result<ElementDocument, Error> {
        let raw = unsafe {
            rsmlui_sys::context::context_load_document(
                self.handle.as_mut_ptr(),
                document_path.into(),
            )
        };

        if raw.is_null() {
            return Err(Error::DocumentCreateFailed);
        }

        Ok(ElementDocument::new_with_borrow(
            // Safety: raw is a valid, non-null pointer
            unsafe { ElementDocumentHandle::from_raw(raw) },
            self,
        ))
    }

    #[rmldoc(name = "Rml::Context::SetDimensions")]
    pub fn set_dimensions(&mut self, dimensions: impl Into<UVec2>) {
        unsafe {
            rsmlui_sys::context::context_set_dimensions(
                self.handle.as_mut_ptr(),
                dimensions.into().into_sys(),
            )
        }
    }
}
