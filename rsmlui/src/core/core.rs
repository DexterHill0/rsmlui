use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use drop_tree::{DropCtx, drop_tree};
use glam::IVec2;

use crate::core::app::{AppState, ApplicationHandler};
use crate::core::backend::{Backend, BackendRuntime, MonolithicBackend};
use crate::core::context::Context;
use crate::core::events::{KeyboardEvent, WindowEvent};
use crate::errors::RsmlUiError;
use crate::interfaces::renderer::RenderInterface;
use crate::interfaces::system::SystemInterface;
use crate::interfaces::window::WindowInterface;
use crate::interfaces::{InterfaceHandle, InterfaceState};
use crate::not_send_sync;
use crate::types::input::{KeyCode, KeyModifier};
use crate::utils::conversions::IntoSys;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

struct DropBomb;

impl Drop for DropBomb {
    fn drop(&mut self) {
        panic!("it is unsafe to drop an `RsmlUiUninitialized`; it must be initialized first.")
    }
}

pub struct RsmlUiUninitialized<T: 'static = ()> {
    _drop: DropBomb,
    backend: ManuallyDrop<Box<dyn BackendRuntime<T>>>,
}

not_send_sync!(
    [T: 'static] RsmlUiUninitialized[T]
);

impl<T: 'static> RsmlUiUninitialized<T> {
    pub fn new_with_monolithic_backend<B: MonolithicBackend<T> + 'static>(
        backend: B,
    ) -> RsmlUiUninitialized<T> {
        Self {
            _drop: DropBomb,
            backend: ManuallyDrop::new(Box::new(backend)),
        }
    }

    pub fn new_with_custom_backend<W: 'static, S: 'static, R: 'static>(
        backend: Backend<W, S, R, T>,
    ) -> RsmlUiUninitialized<T>
    where
        W: WindowInterface<T>,
        InterfaceState<S>: SystemInterface,
        InterfaceState<R>: RenderInterface,
    {
        Self {
            _drop: DropBomb,
            backend: ManuallyDrop::new(Box::new(backend)),
        }
    }
}

impl<T: 'static> RsmlUiUninitialized<T> {
    pub fn initialize(mut self) -> Result<RsmlUi<T>, RsmlUiError> {
        std::mem::forget(self._drop);

        // TODO: how do handle partial failed initilisation?
        // eg. backend initialised and creates data but then core::initialise fails
        // we ideally want to recover from this

        self.backend.initialize()?;

        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::InitializationFailed);
        }

        if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err(RsmlUiError::AlreadyInitialized);
        }

        Ok(RsmlUi {
            dispatcher: AppDispatcher {
                active: ActiveApp::new_with_borrow(
                    AppState::Stopped,
                    ManuallyDrop::new(ManuallyDrop::into_inner(self.backend)),
                ),
                app: None,
                last_poll: Instant::now(),
            },
        })
    }
}

fn app_destructor<T: 'static>(ctx: DropCtx<ActiveApp<T>>) {
    // core must shutdown before the backend
    rsmlui_sys::core::shutdown();

    unsafe { ManuallyDrop::drop(&mut ctx.backend) };
}

// this will only call the destructor once all resources borrowing from this ownership node
// have themselves dropped
#[drop_tree(destructor(app_destructor))]
pub struct ActiveApp<T: 'static = ()> {
    state: AppState,
    backend: ManuallyDrop<Box<dyn BackendRuntime<T>>>,
}

not_send_sync!([T: 'static] ActiveApp[T]);

pub struct AppDispatcher<T: 'static = ()> {
    active: ActiveApp<T>,
    app: Option<Box<dyn ApplicationHandler<T>>>,
    last_poll: Instant,
}

not_send_sync!([T: 'static] AppDispatcher[T]);

pub struct RsmlUi<T: 'static = ()> {
    dispatcher: AppDispatcher<T>,
}

not_send_sync!([T: 'static] RsmlUi[T]);

impl<T: 'static> RsmlUi<T> {
    pub fn run_app<A: ApplicationHandler<T> + 'static>(
        &mut self,
        app: A,
    ) -> Result<(), RsmlUiError> {
        self.dispatcher.app.replace(Box::new(app));

        let driver = self.dispatcher.active.backend.app_driver();
        driver.run(&mut self.dispatcher)?;

        Ok(())
    }
}

impl<T: 'static> ActiveApp<T> {
    pub fn create_context<N: Into<String>>(
        &self,
        name: N,
        dimensions: IVec2,
    ) -> Result<Context, RsmlUiError> {
        let raw = rsmlui_sys::core::create_context(name.into(), dimensions.into_sys());

        if raw.is_null() {
            return Err(RsmlUiError::ContextCreateFailed);
        }

        Ok(Context::new_with_borrow(raw, self))
    }

    pub fn load_font_face<P: Into<String>>(&self, path: P) -> Result<(), RsmlUiError> {
        if !rsmlui_sys::core::load_font_face(path.into()) {
            return Err(RsmlUiError::FontFaceLoadFailed);
        }

        Ok(())
    }

    #[inline]
    pub fn exit(&mut self) {
        self.state = AppState::Stopped;
        // // we can't just call `self.backend.request_exit()` as that will free the backend object and memory
        // // but it's possible for the while loop to still emit a render event, which would cause a UAF
    }

    #[inline]
    pub fn request_exit(&mut self) {
        self.state = AppState::Stopping;
    }

    #[inline]
    pub fn cancel_exit(&mut self) {
        self.state = AppState::Running;
    }
}

// impl<T: 'static> crate::core::app::sealed::Sealed for RsmlUi<T> {}

// TODO: should the driver be fully in control of the app state?
// on one hand, in order to keep drivers consistent, having the SST of the current app state here makes sense
// also, the drivers dont have to worry about calling the unsafe backend methods
// on the other hand, it's just more complexity and theres probably other ways the state can desync between the two anyway
// this `AppInner` trait will probably need to stay anyway, as it limits *what* the driver has access to (such as not being able to create contexts)
// which is important, so most likely it makes sense for the SST to be on `RsmlUi` rather than each driver
impl<T: 'static> AppDispatcher<T> {
    pub fn starting(&mut self) -> Result<(), RsmlUiError> {
        self.active.state = AppState::Running;

        let mut app = match self.app.take() {
            Some(app) => app,
            // TODO: warn on missing app
            None => return Ok(()),
        };

        app.starting(&mut self.active)?;

        self.app.replace(app);

        Ok(())
    }

    pub fn handle_event(&mut self, event: WindowEvent<T>) -> Result<(), RsmlUiError> {
        let mut app = match self.app.take() {
            Some(app) => app,
            None => return Ok(()),
        };

        match event {
            WindowEvent::ExitRequested => {
                if self.active.state != AppState::Stopping {
                    self.active.state = AppState::Stopping;
                    app.event(event, &mut self.active)?;
                }
            },
            WindowEvent::ExitCancelled => {
                if self.active.state == AppState::Stopping {
                    self.active.state = AppState::Running;
                    app.event(event, &mut self.active)?;
                }
            },
            _ => app.event(event, &mut self.active)?,
        }

        self.app.replace(app);

        Ok(())
    }

    pub fn request_render(&mut self) -> Result<(), RsmlUiError> {
        if matches!(self.active.state, AppState::Stopped) {
            return Ok(());
        }

        let mut app = match self.app.take() {
            Some(app) => app,
            None => return Ok(()),
        };

        let now = Instant::now();
        let delta = now - self.last_poll;
        self.last_poll = now;

        app.event(WindowEvent::UpdateRequested, &mut self.active)?;

        // SAFETY: For this function to be called the backend will have been initialized.
        unsafe { self.active.backend.begin_frame() };

        app.event(WindowEvent::RenderRequested(delta), &mut self.active)?;

        // SAFETY: Same as above
        unsafe {
            self.active.backend.present_frame();
        }

        self.app.replace(app);

        Ok(())
    }

    pub fn should_exit(&self) -> bool {
        matches!(self.active.state, AppState::Stopped)
    }

    pub(crate) fn get_context(&mut self) -> Option<&mut Context> {
        let app = match self.app.as_mut() {
            Some(app) => app,
            None => return None,
        };

        app.get_context()
    }
}
