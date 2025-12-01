use std::marker::PhantomData;

use glam::IVec2;

use crate::core::context::Context;
use crate::errors::RsmlUiError;
use crate::interfaces::RawInterface;
use crate::interfaces::backend::Backend;
use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::system::SystemInterfaceMarker;
use crate::utils::conversions::IntoSys;

// impl Default for InterfaceStore {
//     fn default() -> Self {
//         Self {
//             system: None,
//             render: None,
//         }
//     }
// }

pub struct RsmlUi<'app, B: Backend + 'app> {
    backend: Option<B>,
    system_interface: Option<RawInterface<SystemInterfaceMarker>>,
    _phantom: PhantomData<&'app ()>,
}

impl<'app, B: Backend> RsmlUi<'app, B> {
    /// Initializes RmlUi. Must be called after setting interfaces but before creating contexts.
    pub fn initialise() -> Result<Self, RsmlUiError> {
        // currently initialisation only returns a bool, and `false` is only returned when a font engine is missing
        // in the future, if more failure points are added to RmlUi hopefully this will change from a bool
        // to something else to help identify which part failed, aside from just the logs
        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::InitializationFailed);
        }

        Ok(Self {
            backend: None,
            system_interface: None,
            _phantom: PhantomData, // interfaces: Default::default(),
        })
    }

    pub fn get_version() -> String {
        rsmlui_sys::core::get_version()
    }

    pub fn create_context<T: Into<String>>(
        &'app self,
        name: T,
        dimensions: IVec2,
    ) -> Option<Context<'app>> {
        let raw = rsmlui_sys::core::create_context(name.into(), dimensions.into_sys());

        if raw.is_null() {
            return None;
        }

        Some(Context {
            raw,
            _phantom: PhantomData,
        })
    }

    pub fn load_font_face<T: Into<String>>(&self, path: T) -> Result<(), RsmlUiError> {
        if !rsmlui_sys::core::load_font_face(path.into()) {
            return Err(RsmlUiError::FontFaceLoadFailed);
        }

        Ok(())
    }

    pub fn use_backend(&mut self, mut backend: B)
    where
        for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
        for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
    {
        if let Some(system_interface) = backend.get_system_interface() {
            let raw: RawInterface<SystemInterfaceMarker> = system_interface.into();

            unsafe { rsmlui_sys::core::set_system_interface(raw.0) };
        }

        if let Some(render_interface) = backend.get_render_interface() {
            let raw: RawInterface<RenderInterfaceMarker> = render_interface.into();

            unsafe { rsmlui_sys::core::set_render_interface(raw.0) };
        }

        self.backend.replace(backend);
    }

    pub fn request_exit(&self) {
        if let Some(backend) = self.backend.as_ref() {
            backend.request_exit();
        }
    }

    pub fn begin_frame(&self) {
        if let Some(backend) = self.backend.as_ref() {
            backend.begin_frame();
        }
    }

    pub fn present_frame(&self) {
        if let Some(backend) = self.backend.as_ref() {
            backend.present_frame();
        }
    }
}

impl<'app, B: Backend> Drop for RsmlUi<'app, B> {
    fn drop(&mut self) {
        rsmlui_sys::core::shutdown();
    }
}
