use std::cell::{RefCell, UnsafeCell};
use std::mem::ManuallyDrop;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::Instant;

use drop_tree::{DropCtx, drop_tree};
use glam::IVec2;

use crate::core::context::Context;
use crate::core::events::{KeyboardEvent, WindowEvent, WindowEventEmitter};
use crate::errors::RsmlUiError;
use crate::interfaces::RawInterface;
use crate::interfaces::backend::Backend;
use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::system::SystemInterfaceMarker;
use crate::not_send_sync;
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

pub trait BoundedBackend = Backend
where
    for<'a> &'a mut <Self as Backend>::SystemInterface: Into<RawInterface<SystemInterfaceMarker>>,
    for<'a> &'a mut <Self as Backend>::RenderInterface: Into<RawInterface<RenderInterfaceMarker>>;

#[derive(Clone, Debug, PartialEq, Eq)]
enum AppState {
    Stopped,
    Stopping,
    Running,
}

pub trait RsmlUiApp<B: BoundedBackend, T: 'static = ()> {
    fn starting(&mut self, app: &mut RsmlUi<B>) -> Result<(), RsmlUiError>;

    fn event(&mut self, event: WindowEvent<T>, app: &mut RsmlUi<B>) -> Result<(), RsmlUiError>;

    fn get_context(&mut self) -> Option<&mut Context>;
}

fn app_destructor<B: BoundedBackend>(ctx: DropCtx<RsmlUi<B>>) {
    // core must shutdown before the backend
    rsmlui_sys::core::shutdown();

    unsafe { ManuallyDrop::drop(&mut ctx.backend) };
}

// this will only call the destructor once all resources borrowing from this ownership node
// have themselves dropped
#[drop_tree(destructor(app_destructor))]
pub struct RsmlUi<B: BoundedBackend> {
    state: AppState,
    backend: ManuallyDrop<B>,
    last_poll: Instant,
}

not_send_sync!([B: BoundedBackend] RsmlUi[B]);

impl<B: BoundedBackend> RsmlUi<B> {
    #[inline(always)]
    fn backend(&self) -> &B {
        unsafe { &self.backend }
    }

    #[inline(always)]
    fn backend_mut(&mut self) -> &mut B {
        unsafe { &mut self.backend }
    }

    /// Initializes RmlUi. Must only be called once.
    pub fn new(mut backend: B) -> Result<Self, RsmlUiError> {
        if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err(RsmlUiError::AlreadyInitialized);
        }

        Self::use_backend(&mut backend);

        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::InitializationFailed);
        }

        // let backend = Rc::new(UnsafeCell::new(backend));

        Ok(Self::new_with_borrow(
            AppState::Stopped,
            // Rc::clone(&backend),
            ManuallyDrop::new(backend),
            Instant::now(),
        ))
    }

    fn run_app_inner<A: RsmlUiApp<B, T>, T: 'static>(
        &mut self,
        app: &mut A,
    ) -> Result<(), RsmlUiError> {
        app.starting(self)?;

        match self.state {
            AppState::Stopped | AppState::Stopping => return Ok(()),
            _ => {},
        }

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

        while matches!(self.state, AppState::Running | AppState::Stopping) {
            let now = Instant::now();
            let detla = now - self.last_poll;
            self.last_poll = now;

            // TODO: should we really still be calling `process_events` while the app is stopping?
            if self.backend_mut().should_poll(detla)
                && let Some(context) = app.get_context()
            {
                self.backend().process_events(context, &sender)?;
            }

            if let Ok(event) = rx.try_recv() {
                match event {
                    WindowEvent::ExitRequested => {
                        if self.state != AppState::Stopping {
                            // must come before `app.event` as the user could call `app.exit` which would override
                            // and set it to `Stopped`
                            self.state = AppState::Stopping;

                            app.event(event, self)?;
                        }
                    },
                    WindowEvent::ExitCancelled => {
                        if self.state == AppState::Stopping {
                            self.state = AppState::Running;

                            app.event(event, self)?;
                        }
                    },
                    _ => {
                        app.event(event, self)?;
                    },
                }
            }

            match self.state {
                AppState::Stopped => break,
                _ => {},
            }

            app.event(WindowEvent::RenderRequested, self)?;
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

        Ok(Context::new_with_borrow(raw, self))
    }

    pub fn load_font_face<T: Into<String>>(&self, path: T) -> Result<(), RsmlUiError> {
        if !rsmlui_sys::core::load_font_face(path.into()) {
            return Err(RsmlUiError::FontFaceLoadFailed);
        }

        Ok(())
    }

    #[inline]
    pub fn exit(&mut self) {
        self.state = AppState::Stopped;
        // we can't just call `self.backend.request_exit()` as that will free the backend object and memory
        // but it's possible for the while loop to still emit a render event, which would cause a UAF
    }

    #[inline]
    pub fn request_exit(&mut self) {
        self.state = AppState::Stopping;
    }

    #[inline]
    pub fn cancel_exit(&mut self) {
        self.state = AppState::Running;
    }

    #[inline]
    pub fn begin_frame(&self) {
        self.backend().begin_frame();
    }

    #[inline]
    pub fn present_frame(&self) {
        self.backend().present_frame();
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
}

// impl<B: BoundedBackend> Drop for RsmlUi<B> {
//     fn drop(&mut self) {
//         unsafe {
//             ManuallyDrop::drop(&mut self.app);

//             // dbg!(&Rc::strong_count(&self.backend));

//             // although `app` holds a strong reference to the backend, it is dropped first, so there
//             // should be no more strong references to the backend at this point
//             // debug_assert!(Rc::strong_count(&self.backend) == 1);

//             // backend MUST be the last thing shutdown
//             // ManuallyDrop::drop(&mut self.backend);
//         }
//     }
// }

// impl<B: BoundedBackend + 'static> RsmlUi<B> {
// /// Initializes RmlUi. Must only be called once.
// pub fn new(mut backend: B) -> Result<Self, RsmlUiError> {
//     if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
//         return Err(RsmlUiError::AlreadyInitialized);
//     }

//     Self::use_backend(&mut backend);

//     if !rsmlui_sys::core::initialise() {
//         return Err(RsmlUiError::InitializationFailed);
//     }

//     // let backend = Rc::new(UnsafeCell::new(backend));

//     Ok(Self::new_with_borrow(
//         AppState::Stopped,
//         // Rc::clone(&backend),
//         ManuallyDrop::new(backend),
//         Instant::now(),
//     ))
// }

//     pub fn get_version() -> String {
//         rsmlui_sys::core::get_version()
//     }

//     pub fn run_app<A: RsmlUiApp<B, T>, T: 'static>(
//         &mut self,
//         app: &mut A,
//     ) -> Result<(), RsmlUiError> {
//         self.app.run_app(app)
//     }

// pub(crate) fn use_backend(backend: &mut B) {
//     if let Some(system_interface) = backend.get_system_interface() {
//         let raw: RawInterface<SystemInterfaceMarker> = system_interface.into();

//         unsafe { rsmlui_sys::core::set_system_interface(raw.0) };
//     }

//     if let Some(render_interface) = backend.get_render_interface() {
//         let raw: RawInterface<RenderInterfaceMarker> = render_interface.into();

//         unsafe { rsmlui_sys::core::set_render_interface(raw.0) };
//     }
// }

//     /// Exits the app immediately. If you want a graceful exit, use [`Self::request_exit`] instead.
//     #[inline]
//     pub fn exit(&mut self) {
//         self.app.exit();
//     }

//     /// Requests the app to shutdown gracefully. This will *not* emit a `WindowEvent::ExitRequested` event.
//     #[inline]
//     pub fn request_exit(&mut self) {
//         self.app.request_exit();
//     }

//     /// Cancels a previously requested exit. This doesn't do anything if an exit was not requested. This will *not* emit a `WindowEvent::ExitCancelled` event.
//     #[inline]
//     pub fn cancel_exit(&mut self) {
//         self.app.cancel_exit();
//     }

//     #[inline]
//     pub fn begin_frame(&self) {
//         self.app.begin_frame();
//     }

//     #[inline]
//     pub fn present_frame(&self) {
//         self.app.present_frame();
//     }
// }
