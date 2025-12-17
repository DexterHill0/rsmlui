use std::sync::mpsc::Sender;

use crate::errors::RsmlUiError;
use crate::utils::input::{KeyCode, KeyModifier};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum KeyboardEvent {
    KeyPressed {
        code: KeyCode,
        modifier: KeyModifier,
        native_dp_ratio: f32,
        /// A key press event is first tested against high-priority global shortcuts, so this is `false`,
        /// but if nothing consumes the event it is set to `true` to match against fallback/lower-priority shortcuts.
        fallback: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent<T: 'static = ()> {
    ExitRequested,
    RenderRequested,

    KeyboardEvent(KeyboardEvent),

    UserEvent(T),
}

pub struct WindowEventEmitter<T: 'static = ()>(pub(crate) Sender<WindowEvent<T>>);

impl<T: 'static> Clone for WindowEventEmitter<T> {
    fn clone(&self) -> Self {
        WindowEventEmitter(self.0.clone())
    }
}

impl<T: 'static> WindowEventEmitter<T> {
    pub fn emit(&self, event: WindowEvent<T>) -> Result<(), RsmlUiError> {
        self.0.send(event).map_err(|_| RsmlUiError::EventSendFailed)
    }
}
