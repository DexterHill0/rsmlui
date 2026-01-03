use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use glam::IVec2;

use crate::core::app::{AppDriver, ApplicationHandler};
use crate::core::context::Context;
use crate::errors::RsmlUiError;
use crate::interfaces::RawInterface;
use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::system::{SystemInterface, SystemInterfaceMarker};
use crate::interfaces::window::WindowInterface;

pub(crate) mod sealed {
    #[doc(hidden)]
    pub trait Sealed {}
}

pub(crate) trait BackendRuntime<T: 'static = ()>: sealed::Sealed {
    fn app_driver(&mut self) -> Box<dyn AppDriver<T>>;

    fn initialize(&mut self) -> Result<(), RsmlUiError>;
    /// # Safety
    ///
    /// The caller must ensure [`BackendRuntime::initialize`] has been called.
    unsafe fn begin_frame(&mut self);
    /// # Safety
    ///
    /// The caller must ensure [`BackendRuntime::initialize`] has been called.
    unsafe fn present_frame(&mut self);
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

pub struct BackendBuilder<W, S = (), R = ()> {
    window: W,
    system: Option<S>,
    render: Option<R>,
}

// impl<W: WindowInterface> BackendBuilder<W, (), ()> {
//     pub fn new_with_window(window: W) -> Self {
//         Self {
//             window,
//             system: None,
//             render: None,
//         }
//     }
// }

impl<W, S, R> BackendBuilder<W, S, R> {
    pub fn with_system() {}

    pub fn with_system_uninstanced() {}

    pub fn build(self) -> Backend<W, S, R> {
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
