use std::time::Duration;

use crate::core::context::Context;
use crate::core::core::{ActiveApp, AppDispatcher, RsmlUi};
use crate::core::events::WindowEvent;
use crate::errors::RsmlUiError;

pub(crate) mod sealed {
    #[doc(hidden)]
    #[diagnostic::on_unimplemented(message = "This trait should not be implemented manually.")]
    pub trait Sealed {}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum AppState {
    Stopped,
    Stopping,
    Running,
}

// user trait
pub trait ApplicationHandler<T: 'static = ()> {
    fn starting(&mut self, app: &mut ActiveApp<T>) -> Result<(), RsmlUiError>;

    fn event(&mut self, event: WindowEvent<T>, app: &mut ActiveApp<T>) -> Result<(), RsmlUiError>;

    fn get_context(&mut self) -> Option<&mut Context>;
}

// //
// pub trait AppDispatcher<T: 'static>: sealed::Sealed {
//     fn starting(&mut self) -> Result<(), RsmlUiError>;

//     fn handle_event(&mut self, event: WindowEvent<T>) -> Result<(), RsmlUiError>;

//     fn request_render(&mut self, delta: Duration) -> Result<(), RsmlUiError>;

//     fn should_exit(&self) -> bool;
// }

pub trait AppDriver<T: 'static> {
    fn run(&mut self, app: &mut AppDispatcher<T>) -> Result<(), RsmlUiError>;
}
