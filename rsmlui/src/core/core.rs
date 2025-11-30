use glam::IVec2;
use rsmlui_sys::utils::IntoPtr;

use crate::{
    core::context::Context,
    errors::RsmlUiError,
    interfaces::{
        renderer::{IntoRenderInterfacePtr, RenderInterface, RenderInterfaceExtAdapter},
        system::{IntoSystemInterfacePtr, SystemInterface, SystemInterfaceExtAdapter},
    },
    utils::conversions::IntoSys,
};

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

    pub fn set_system_interface<I: IntoSystemInterfacePtr>(&mut self, system_interface: I) {
        unsafe { rsmlui_sys::core::set_system_interface(system_interface.into_ptr()) }
    }

    pub fn set_render_interface<R: IntoRenderInterfacePtr>(&mut self, render_interface: R) {
        unsafe { rsmlui_sys::core::set_render_interface(render_interface.into_ptr()) }
    }
}

impl Drop for RsmlUi {
    fn drop(&mut self) {
        rsmlui_sys::core::shutdown();
    }
}
