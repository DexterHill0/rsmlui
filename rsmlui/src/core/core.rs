use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::Instant;

use drop_tree::{DropCtx, drop_tree};
use glam::IVec2;

use crate::core::context::Context;
use crate::core::events::{KeyboardEvent, WindowEvent, WindowEventEmitter};
use crate::errors::RsmlUiError;
use crate::interfaces::backend::{BackendRuntime, MonolithicBackend};
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum AppState {
    Stopped,
    Stopping,
    Running,
}

pub trait RsmlUiApp<T: 'static = ()> {
    fn starting(&mut self, app: &mut RsmlUi<T>) -> Result<(), RsmlUiError>;

    fn event(&mut self, event: WindowEvent<T>, app: &mut RsmlUi<T>) -> Result<(), RsmlUiError>;

    fn get_context(&mut self) -> Option<&mut Context>;
}

fn app_destructor<T: 'static>(ctx: DropCtx<RsmlUi<T>>) {
    // core must shutdown before the backend
    rsmlui_sys::core::shutdown();

    unsafe { ManuallyDrop::drop(&mut ctx.backend) };
}

// this will only call the destructor once all resources borrowing from this ownership node
// have themselves dropped
#[drop_tree(destructor(app_destructor))]
pub struct RsmlUi<T: 'static = ()> {
    state: AppState,
    backend: ManuallyDrop<Box<dyn BackendRuntime<T>>>,
    last_poll: Instant,
}

not_send_sync!([T: 'static] RsmlUi[T]);

struct DropBomb;

impl Drop for DropBomb {
    fn drop(&mut self) {
        panic!("an `RsmlUiBuilder` must not be dropped; `build` must be called instead.")
    }
}

pub struct RsmlUiBuilder<B, T: 'static = ()> {
    _drop: DropBomb,
    backend: ManuallyDrop<Box<dyn BackendRuntime<T>>>,
    _phantom: PhantomData<B>,
}

not_send_sync!(
    [M, T: 'static] RsmlUiBuilder[M, T]
);

impl<T: 'static> RsmlUiBuilder<(), T> {
    pub fn new_with_monolithic_backend<B: MonolithicBackend<T> + 'static>(
        backend: B,
    ) -> RsmlUiBuilder<B, T> {
        Self::new(backend)
    }

    // pub fn new_with_backend(backend: B) -> RsmlUiBuilder<B> {
    //     Self::new(backend)
    // }

    fn new<B: BackendRuntime<T> + 'static>(backend: B) -> RsmlUiBuilder<B, T> {
        RsmlUiBuilder {
            _drop: DropBomb,
            backend: ManuallyDrop::new(Box::new(backend)),
            _phantom: PhantomData,
        }
    }
}

// impl<B: MonolithicBackend, T: 'static> RsmlUiBuilder<B, T> {}

// impl<B: Backend, T: 'static> RsmlUiBuilder<B, T> {}

impl<B, T: 'static> RsmlUiBuilder<B, T> {
    pub fn build(mut self) -> Result<RsmlUi<T>, RsmlUiError> {
        std::mem::forget(self._drop);

        if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
            return Err(RsmlUiError::AlreadyInitialized);
        }

        self.backend.initialize()?;

        if !rsmlui_sys::core::initialise() {
            return Err(RsmlUiError::InitializationFailed);
        }

        Ok(RsmlUi::new_with_borrow(
            AppState::Stopped,
            self.backend,
            Instant::now(),
        ))
    }
}

impl<T: 'static> RsmlUi<T> {
    // #[inline(always)]
    // fn window(&mut self) -> &mut B::Window {
    //     // self.backend.window()
    // }

    fn run_app_inner<A: RsmlUiApp<T>>(&mut self, app: &mut A) -> Result<(), RsmlUiError> {
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
            let delta = now - self.last_poll;
            self.last_poll = now;

            if let Some(context) = app.get_context() {
                self.backend.poll_events(&sender, context, delta)?;
            }

            while let Ok(event) = rx.try_recv() {
                match event {
                    WindowEvent::ExitRequested => {
                        if self.state != AppState::Stopping {
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
                    _ => app.event(event, self)?,
                }
            }

            if matches!(self.state, AppState::Stopped) {
                break;
            }

            self.backend.begin_frame();

            app.event(WindowEvent::RenderRequested, self)?;

            self.backend.present_frame();
        }

        Ok(())
    }

    pub fn run_app<A: RsmlUiApp<T>>(&mut self, app: &mut A) -> Result<(), RsmlUiError> {
        self.state = AppState::Running;

        let run_result = self.run_app_inner(app);

        self.state = AppState::Stopped;

        BACKEND_EVENTS_CALLBACK.replace(None);

        run_result
    }

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
