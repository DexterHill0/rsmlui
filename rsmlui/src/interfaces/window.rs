use std::time::Duration;

use glam::IVec2;

use crate::core::app::{AppDriver, ApplicationHandler};
use crate::errors::RsmlUiError;
use crate::types::cursor::Cursor;

// TODO: comment that this is not a native rmlui interface
pub trait WindowInterface<T: 'static> {
    fn initialize(&mut self) -> Result<(), RsmlUiError>;

    fn driver(&mut self) -> &mut dyn AppDriver<T>;

    // fn poll_events(
    //     &mut self,
    //     sender: &WindowEventEmitter<T>,
    //     delta: Duration,
    // ) -> Result<(), RsmlUiError>;

    // fn should_close(&self) -> bool;

    // fn begin_frame(&mut self);

    // fn present_frame(&mut self);

    // fn dimensions(&self) -> IVec2;

    // fn set_cursor(&mut self, cursor: Cursor);
}

// pub trait WindowBuilder {
//     type Window: WindowInterface;

//     fn create_window(self) -> Result<Self::Window, RsmlUiError>;
// }
