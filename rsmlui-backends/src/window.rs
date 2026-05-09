use std::error::Error as StdError;

use glam::IVec2;
use rsmlui_core::core::context::Context;

/// Allows custom windowing and render-loop implementations.
///
/// This trait only supports compositional [`Backend`]s
/// as monolithic backends (e.g [`Win32Gl2Backend`]) have
/// windowing logic implemented directly in C++.
///
/// The three frame methods are called in order each tick:
/// 1. [`process_events`]: deliver OS events to the RmlUi context
/// 2. [`begin_frame`]: set up the frame ready for rendering
/// 3. [`present_frame`]: show the frame to the screen
///
/// # Example
///
/// TODO: example
///
/// [`Backend`]: crate::backend::Backend
/// [`Win32Gl2Backend`]: crate::monolithic::Win32Gl2Backend
/// [`process_events`]: WindowDriver::process_events
/// [`begin_frame`]: WindowDriver::begin_frame
/// [`present_frame`]: WindowDriver::present_frame
pub trait WindowDriver {
    type Error: StdError;

    fn process_events(&mut self, context: &Context) -> Result<bool, Self::Error>;

    /// Prepare for rendering (e.g. clear the framebuffer).
    fn begin_frame(&mut self) -> Result<(), Self::Error>;

    /// Present the rendered frame (e.g. swap GL buffers).
    fn present_frame(&mut self) -> Result<(), Self::Error>;

    /// Current window dimensions in physical pixels.
    fn dimensions(&self) -> IVec2;
}
