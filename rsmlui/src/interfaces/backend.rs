use std::time::Duration;


use crate::core::context::Context;
use crate::core::events::WindowEventEmitter;
use crate::errors::RsmlUiError;
use crate::interfaces::window::WindowInterface;

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub trait BackendRuntime<T: 'static = ()>: sealed::Sealed {
    fn initialize(&mut self) -> Result<(), RsmlUiError>;

    fn begin_frame(&mut self);

    fn present_frame(&mut self);

    fn poll_events(
        &mut self,
        sender: &WindowEventEmitter<T>,
        context: &mut Context,
        delta: Duration,
    ) -> Result<(), RsmlUiError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BackendOptions {
    pub allow_resize: bool,
    pub power_save: bool,
}

pub struct Backend<W, S, R> {
    pub window: W,
    pub system: Option<S>,
    pub render: Option<R>,
}

struct DropBomb;

impl Drop for DropBomb {
    fn drop(&mut self) {
        panic!("a `BackendBuilder` must not be dropped; `build` must be called instead.")
    }
}

pub struct BackendBuilder<W, S = (), R = ()> {
    _drop: DropBomb,
    window: W,
    system: Option<S>,
    render: Option<R>,
}

impl<W: WindowInterface> BackendBuilder<W, (), ()> {
    pub fn new_with_window(window: W) -> Self {
        Self {
            _drop: DropBomb,
            window,
            system: None,
            render: None,
        }
    }
}

impl<W, S, R> BackendBuilder<W, S, R> {
    pub fn with_system() {}

    pub fn with_system_uninstanced() {}

    pub fn build(self) -> Backend<W, S, R> {
        std::mem::forget(self._drop);

        Backend {
            window: self.window,
            system: self.system,
            render: self.render,
        }
    }
}
// pub trait Backend: BackendRuntime {
//     type Window: WindowInterface;
//     type System: Into<RawInterface<SystemInterfaceMarker>>;
//     type Render: Into<RawInterface<RenderInterfaceMarker>>;

//     fn window(&mut self) -> &mut Self::Window;
//     fn system(&mut self) -> Option<&mut Self::System>;
//     fn render(&mut self) -> Option<&mut Self::Render>;
// }

// impl<B: Backend, T: 'static> BackendRuntime<T> for B {
//     fn initialize(&mut self) -> Result<(), RsmlUiError> {
//         todo!()
//     }

//     fn begin_frame(&mut self) {
//         todo!()
//     }

//     fn present_frame(&mut self) {
//         todo!()
//     }

//     fn poll_events(
//         &mut self,
//         sender: &WindowEventEmitter<T>,
//         delta: Duration,
//     ) -> Result<(), RsmlUiError> {
//         todo!()
//     }
// }

pub trait MonolithicBackend<T: 'static>: BackendRuntime<T> {}
