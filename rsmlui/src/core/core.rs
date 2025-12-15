use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use glam::IVec2;

use crate::core::context::Context;
use crate::core::events::WindowEvent;
use crate::errors::RsmlUiError;
use crate::interfaces::RawInterface;
use crate::interfaces::backend::Backend;
use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::system::SystemInterfaceMarker;
use crate::utils::conversions::IntoSys;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub trait RsmlUiApp<B: Backend>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    fn starting(&mut self, ui: &mut RsmlUi<B>) -> Result<(), RsmlUiError>;

    fn event(&mut self, event: WindowEvent, ui: &mut RsmlUi<B>) -> Result<(), RsmlUiError>;
}

// marker trait owned by the RsmlUi and cloned onto contexts, etc
// its purpose is to keep the app alive while those constructs are still alive
// although the `RsmlUi` value can be dropped, RmlUi itself won't be shutdown until
// all resources belonging to the app have been destroyed too
pub(crate) struct AppOwner;

impl Drop for AppOwner {
    fn drop(&mut self) {
        rsmlui_sys::core::shutdown();
    }
}

pub struct RsmlUi<B: Backend>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    backend: B,
    _owner: Rc<AppOwner>,
}

impl<B: Backend> RsmlUi<B>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    /// Initializes RmlUi. Must only be called once.
    pub fn new(mut backend: B) -> Result<Self, RsmlUiError> {
        Self::use_backend(&mut backend);

        if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err(RsmlUiError::AlreadyInitialized);
        }

        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::InitializationFailed);
        }

        Ok(Self {
            backend,
            _owner: Rc::new(AppOwner),
        })
    }

    pub fn get_version() -> String {
        rsmlui_sys::core::get_version()
    }

    pub fn run_app<A: RsmlUiApp<B>>(&mut self, app: &mut A) -> Result<(), RsmlUiError> {
        app.starting(self)?;

        loop {
            app.event(WindowEvent::RenderRequested, self)?;
        }
    }

    pub(crate) fn use_backend(backend: &mut B) {
        if let Some(system_interface) = backend.get_system_interface() {
            let raw: RawInterface<SystemInterfaceMarker> = system_interface.into();

            unsafe { rsmlui_sys::core::set_system_interface(raw.0) };
        }

        if let Some(render_interface) = backend.get_render_interface() {
            let raw: RawInterface<RenderInterfaceMarker> = render_interface.into();

            unsafe { rsmlui_sys::core::set_render_interface(raw.0) };
        }
    }

    pub fn create_context<T: Into<String>>(
        &self,
        name: T,
        dimensions: IVec2,
    ) -> Result<Context, RsmlUiError> {
        let raw = rsmlui_sys::core::create_context(name.into(), dimensions.into_sys());

        if raw.is_null() {
            return Err(RsmlUiError::ContextCreateFailed);
        }

        Ok(Context::from_raw(raw, &self._owner))
    }

    pub fn load_font_face<T: Into<String>>(&self, path: T) -> Result<(), RsmlUiError> {
        if !rsmlui_sys::core::load_font_face(path.into()) {
            return Err(RsmlUiError::FontFaceLoadFailed);
        }

        Ok(())
    }

    #[inline]
    pub fn request_exit(&self) {
        self.backend.request_exit();
    }

    #[inline]
    pub fn begin_frame(&self) {
        self.backend.begin_frame();
    }

    #[inline]
    pub fn present_frame(&self) {
        self.backend.present_frame();
    }
}
