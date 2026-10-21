use drop_tree::drop_tree;
use rsmlui_macros::rmldoc;

use crate::_private::{IntoSys, not_send_sync};
use crate::core::element_document::{ElementDocument, ElementDocumentHandle};
use crate::error::Error;
use crate::math::{IVec2, UVec2, Vec2};
use crate::object::constness::NonConst;
use crate::object::foreign::{ForeignHandle, ForeignObject, ObjectMutPtr};
use crate::object::ownership::Foreign;
use crate::types::event::EventFlow;
use crate::types::input::{KeyCode, KeyModifier, MouseButton};

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

    #[rmldoc(name = "Rml::Context::ProcessMouseMove")]
    pub fn process_mouse_move<T: Into<IVec2>>(
        &mut self,
        position: T,
        key_modifier_state: KeyModifier,
    ) -> bool {
        let position: IVec2 = position.into();

        unsafe {
            rsmlui_sys::context::context_process_mouse_move(
                self.handle.as_mut_ptr(),
                position.x,
                position.y,
                key_modifier_state.into_sys(),
            )
        }
    }

    #[rmldoc(name = "Rml::Context::ProcessMouseButtonDown")]
    pub fn process_mouse_button_down(
        &mut self,
        button_index: MouseButton,
        key_modifier_state: KeyModifier,
    ) -> bool {
        unsafe {
            rsmlui_sys::context::context_process_mouse_button_down(
                self.handle.as_mut_ptr(),
                button_index.into(),
                key_modifier_state.into_sys(),
            )
        }
    }

    #[rmldoc(name = "Rml::Context::ProcessMouseButtonUp")]
    pub fn process_mouse_button_up(
        &mut self,
        button_index: MouseButton,
        key_modifier_state: KeyModifier,
    ) -> bool {
        unsafe {
            rsmlui_sys::context::context_process_mouse_button_up(
                self.handle.as_mut_ptr(),
                button_index.into(),
                key_modifier_state.into_sys(),
            )
        }
    }

    #[rmldoc(
        name = "Rml::Context::ProcessMouseWheel",
        refid = "class_rml_1_1_context_1a3e3f4b6b4fa42e8a069f886849b14ac8"
    )]
    pub fn process_mouse_wheel<T: Into<Vec2>>(
        &mut self,
        wheel_delta: T,
        key_modifier_state: KeyModifier,
    ) -> EventFlow {
        let wheel_delta: Vec2 = wheel_delta.into();

        unsafe {
            rsmlui_sys::context::context_process_mouse_wheel(
                self.handle.as_mut_ptr(),
                wheel_delta.into_sys(),
                key_modifier_state.into_sys(),
            )
            .into()
        }
    }

    #[rmldoc(name = "Rml::Context::ProcessMouseLeave")]
    pub fn process_mouse_leave(&mut self) -> EventFlow {
        unsafe { rsmlui_sys::context::context_process_mouse_leave(self.handle.as_mut_ptr()).into() }
    }

    #[rmldoc(name = "Rml::Context::ProcessKeyDown")]
    pub fn process_key_down(
        &mut self,
        key_identifier: KeyCode,
        key_modifier_state: KeyModifier,
    ) -> EventFlow {
        unsafe {
            rsmlui_sys::context::context_process_key_down(
                self.handle.as_mut_ptr(),
                key_identifier.into_sys(),
                key_modifier_state.into_sys(),
            )
            .into()
        }
    }

    #[rmldoc(name = "Rml::Context::ProcessKeyUp")]
    pub fn process_key_up(
        &mut self,
        key_identifier: KeyCode,
        key_modifier_state: KeyModifier,
    ) -> EventFlow {
        unsafe {
            rsmlui_sys::context::context_process_key_up(
                self.handle.as_mut_ptr(),
                key_identifier.into_sys(),
                key_modifier_state.into_sys(),
            )
            .into()
        }
    }

    #[rmldoc(
        name = "Rml::Context::ProcessTextInput",
        refid = "class_rml_1_1_context_1a1eb37c3339a44947644ed8f57ee5bb3e"
    )]
    pub fn process_text_input_char(&mut self, character: char) -> EventFlow {
        unsafe {
            rsmlui_sys::context::context_process_text_input_char(
                self.handle.as_mut_ptr(),
                character as i8,
            )
            .into()
        }
    }

    #[rmldoc(
        name = "Rml::Context::ProcessTextInput",
        refid = "class_rml_1_1_context_1a2f34194ca96c72923accbc4c28c6cbb7"
    )]
    pub fn process_text_input_string<T: Into<String>>(&mut self, string: T) -> EventFlow {
        let string: String = string.into();

        unsafe {
            rsmlui_sys::context::context_process_text_input_string(
                self.handle.as_mut_ptr(),
                &string,
            )
            .into()
        }
    }
}
