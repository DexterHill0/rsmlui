use glam::IVec2;

use crate::{core::context::Context, errors::RsmlUiError, interfaces::system::SystemInterface};

pub fn get_version() -> String {
    rsmlui_sys::core::get_version();
}

// TODO: should this return a value with a Drop impl that calls shutdown if dropped?
/// Initializes RmlUi. Must be called after setting interfaces but before creating contexts.
pub fn initialise() -> Result<(), RsmlUiError> {
    // currently initialisation only returns a bool, and `false` is only returned when a font engine is missing
    // in the future, if more failure points are added to RmlUi hopefully this will change from a bool
    // to something else to help identify which part failed, aside from just the logs
    if !rsmlui_sys::core::initialise() {
        return Err(RsmlUiError::MissingFontEngine);
    }

    Ok(())
}

pub fn shutdown() {
    rsmlui_sys::core::shutdown();
}

pub fn create_context(name: String, dimensions: IVec2) -> Context {
    let raw = rsmlui_sys::core::create_context(name, dimensions.into());

    Context { raw }
}

pub fn set_system_interface<I: SystemInterface>(system_interface: I) {}

// pub fn set_render_interface(render_interface: *mut RenderInterface);
