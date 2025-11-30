use glam::IVec2;

use crate::core::context::Context;
use crate::errors::RsmlUiError;
use crate::interfaces::backend::{Backend, BackendGuard};
use crate::interfaces::renderer::IntoRenderInterfacePtr;
use crate::interfaces::system::IntoSystemInterfacePtr;
use crate::utils::conversions::IntoSys;

pub struct RsmlUi;

impl RsmlUi {
    /// Initializes RmlUi. Must be called after setting interfaces but before creating contexts.
    pub fn initialise() -> Result<Self, RsmlUiError> {
        // currently initialisation only returns a bool, and `false` is only returned when a font engine is missing
        // in the future, if more failure points are added to RmlUi hopefully this will change from a bool
        // to something else to help identify which part failed, aside from just the logs
        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::MissingFontEngine);
        }

        Ok(Self)
    }

    pub fn get_version() -> String {
        rsmlui_sys::core::get_version()
    }

    pub fn create_context<T: Into<String>>(
        &mut self,
        name: T,
        dimensions: IVec2,
    ) -> Option<Context> {
        let raw = rsmlui_sys::core::create_context(name.into(), dimensions.into_sys());

        if raw.is_null() {
            return None;
        }

        Some(Context { raw })
    }

    pub fn load_font_face<T: Into<String>>(&mut self, path: T) -> Result<(), RsmlUiError> {
        if !rsmlui_sys::core::load_font_face(path.into()) {
            return Err(RsmlUiError::FontFaceLoadFailed);
        }

        Ok(())
    }

    pub fn set_system_interface<I: IntoSystemInterfacePtr>(&mut self, system_interface: I) {
        unsafe { rsmlui_sys::core::set_system_interface(system_interface.into_ptr()) }
    }

    pub fn set_render_interface<R: IntoRenderInterfacePtr>(&mut self, render_interface: R) {
        unsafe { rsmlui_sys::core::set_render_interface(render_interface.into_ptr()) }
    }

    pub fn use_backend<B: Backend>(&mut self, backend: &mut BackendGuard<B>) {
        if let Some(render_interface) = backend.get_render_interface() {
            self.set_render_interface(render_interface);
        }

        if let Some(system_interface) = backend.get_system_interface() {
            self.set_system_interface(system_interface);
        }
    }
}

impl Drop for RsmlUi {
    fn drop(&mut self) {
        rsmlui_sys::core::shutdown();
    }
}
