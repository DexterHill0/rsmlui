use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::errors::RsmlUiError;
use crate::types::input::{KeyCode, KeyModifier};

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
    ExitCancelled,
    UpdateRequested,
    RenderRequested(Duration),

    KeyboardEvent(KeyboardEvent),

    UserEvent(T),
}
