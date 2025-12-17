use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;

use glam::IVec2;

use crate::core::context::Context;
use crate::core::events::{KeyboardEvent, WindowEvent, WindowEventEmitter};
use crate::errors::RsmlUiError;
use crate::interfaces::RawInterface;
use crate::interfaces::backend::Backend;
use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::system::SystemInterfaceMarker;
use crate::utils::conversions::IntoSys;
use crate::utils::input::{KeyCode, KeyModifier};

// impractical, but while RmlUI doesn't have a user data pointer in the callback, this is required
// FIXME: remove once RmlUi has user data pointer
thread_local! {
    pub(crate) static BACKEND_EVENTS_CALLBACK: RefCell<
        Option<Box<dyn for<'ctx> FnMut(KeyCode, KeyModifier, f32, bool) -> bool>>
    > = RefCell::new(None);
}

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(PartialEq, Eq)]
enum AppState {
    Stopped,
    Stopping,
    Running,
}

pub trait RsmlUiApp<B: Backend, T: 'static = ()>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    fn starting(&mut self, app: &mut ActiveApp<B>) -> Result<(), RsmlUiError>;

    fn event(&mut self, event: WindowEvent<T>, app: &mut ActiveApp<B>) -> Result<(), RsmlUiError>;

    fn get_context(&mut self) -> Option<&mut Context>;
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

// the structs setup like this as technically the backend is the real owner of the app, as the backend must be the last thing that ever gets shutdown
// building the structs this way means the drop implementation can enforce that backend is always shutdown last, at the cost of having two different structs
// NOTE: I personally like the winit style, so I *should* like `ActiveApp` (as it's like `ActiveEventLoop`), but if we could get rid of it that would be nicer
pub struct ActiveApp<B: Backend>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    state: AppState,
    backend: Rc<B>,
    _owner: Rc<AppOwner>, // app still has ownership over contexts, etc, so it still has a marker even though itself is "owned" by the backend
    _phantom: PhantomData<B>,
}

pub struct RsmlUi<B: Backend>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    backend: ManuallyDrop<Rc<B>>,
    app: ManuallyDrop<ActiveApp<B>>,
}

impl<B: Backend> ActiveApp<B>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    fn run_app_inner<A: RsmlUiApp<B, T>, T: 'static>(
        &mut self,
        app: &mut A,
    ) -> Result<(), RsmlUiError> {
        app.starting(self)?;

        let (tx, rx) = channel::<WindowEvent<T>>();
        let sender = WindowEventEmitter(tx);

        let sender_inner = sender.clone();

        BACKEND_EVENTS_CALLBACK.replace(Some(Box::new(
            move |code, modifier, native_dp_ratio, priority| {
                // FIXME: remove expect?
                sender_inner
                    .clone()
                    .emit(WindowEvent::KeyboardEvent(KeyboardEvent::KeyPressed {
                        code,
                        modifier,
                        native_dp_ratio,
                        fallback: !priority,
                    }))
                    .expect("failed to send KeyPress event");

                true
            },
        )));

        while self.state == AppState::Running {
            if let Some(context) = app.get_context() {
                self.backend.process_events(context, &sender)?;
            }

            if let Ok(event) = rx.try_recv() {
                app.event(event, self)?;
            }

            app.event(WindowEvent::RenderRequested, self)?;
        }

        match self.state {
            AppState::Stopping => {
                self.backend.request_exit();
            },
            _ => {},
        }

        Ok(())
    }

    pub fn run_app<A: RsmlUiApp<B, T>, T: 'static>(
        &mut self,
        app: &mut A,
    ) -> Result<(), RsmlUiError> {
        self.state = AppState::Running;

        let run_result = self.run_app_inner(app);

        self.state = AppState::Stopped;

        BACKEND_EVENTS_CALLBACK.replace(None);

        run_result
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
    pub fn request_exit(&mut self) {
        self.state = AppState::Stopping;
        // we can't just call `self.backend.request_exit()` as that will free the backend object and memory
        // but it's possible for the while loop to still emit a render event, which would cause a UAF
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

impl<B: Backend> Drop for RsmlUi<B>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.app);

            // although `app` holds a strong reference to the backend, it is dropped first, so there
            // should be no more strong references to the backend at this point
            debug_assert!(Rc::strong_count(&self.backend) == 1);

            // backend MUST be the last thing shutdown
            ManuallyDrop::drop(&mut self.backend);
        }
    }
}

impl<B: Backend> RsmlUi<B>
where
    for<'a> &'a mut B::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut B::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>,
{
    /// Initializes RmlUi. Must only be called once.
    pub fn new(mut backend: B) -> Result<Self, RsmlUiError> {
        if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err(RsmlUiError::AlreadyInitialized);
        }

        Self::use_backend(&mut backend);

        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::InitializationFailed);
        }

        let backend = Rc::new(backend);

        Ok(Self {
            app: ManuallyDrop::new(ActiveApp {
                state: AppState::Stopped,
                backend: Rc::clone(&backend),
                _owner: Rc::new(AppOwner),
                _phantom: PhantomData,
            }),
            backend: ManuallyDrop::new(backend),
        })
    }

    pub fn get_version() -> String {
        rsmlui_sys::core::get_version()
    }

    pub fn run_app<A: RsmlUiApp<B, T>, T: 'static>(
        &mut self,
        app: &mut A,
    ) -> Result<(), RsmlUiError> {
        self.app.run_app(app)
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

    #[inline]
    pub fn request_exit(&mut self) {
        self.app.request_exit();
    }

    #[inline]
    pub fn begin_frame(&self) {
        self.app.begin_frame();
    }

    #[inline]
    pub fn present_frame(&self) {
        self.app.present_frame();
    }
}
