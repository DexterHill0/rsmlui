use std::time::Duration;

use glam::IVec2;

use crate::core::events::WindowEventEmitter;
use crate::errors::RsmlUiError;
use crate::utils::cursor::Cursor;

// TODO: comment that this is not a native rmlui interface
pub trait WindowInterface {
    fn poll_events<T: 'static>(
        &mut self,
        sender: &WindowEventEmitter<T>,
        delta: Duration,
    ) -> Result<(), RsmlUiError>;

    fn should_close(&self) -> bool;

    fn begin_frame(&mut self);

    fn present_frame(&mut self);

    fn dimensions(&self) -> IVec2;

    fn set_cursor(&mut self, cursor: Cursor);
}
