use std::sync::Arc;

use rsmlui_core::interfaces::system::{SystemInterfaceBehaviour, SystemInterfaceObject};
use rsmlui_core::math::Vec2;
use rsmlui_core::types::cursor::Cursor;
use winit::dpi::{PhysicalPosition, PhysicalSize, Position, Size};
use winit::window::{Cursor as WinitCursor, CursorIcon, Window};

/// A [`SystemInterface`] for integrating with `winit`. Implements `set_mouse_cursor`,
/// `activate_keyboard` and `deactivate_keyboard`. All other methods fallback to the
/// default C++ implementation.
///
/// Create a custom [`SystemInterface`] that calls the associated methods in `SystemWinit`
/// if you want to add extra behaviour, such as clipboard support.
///
/// [`SystemInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/system.html
pub struct SystemWinit {
    window: Arc<Window>,
}

impl SystemWinit {
    pub fn set_mouse_cursor(window: &Window, cursor: Cursor) {
        let cursor_icon = WinitCursor::Icon(match cursor {
            Cursor::Arrow => CursorIcon::Default,
            Cursor::Move => CursorIcon::Move,
            Cursor::Pointer => CursorIcon::Pointer,
            Cursor::Resize => CursorIcon::AllResize,
            Cursor::Cross => CursorIcon::Crosshair,
            Cursor::Text => CursorIcon::Text,
            Cursor::Unavailable => CursorIcon::NotAllowed,
            Cursor::Scroll(..) => CursorIcon::AllScroll,
            Cursor::Unknown(..) => CursorIcon::Default,
            _ => CursorIcon::Default,
        });

        window.set_cursor(cursor_icon);
    }

    pub fn activate_keyboard(window: &Window, caret_position: Vec2, line_height: f32) {
        window.set_ime_allowed(true);
        window.set_ime_cursor_area(
            Position::Physical(PhysicalPosition::new(
                caret_position.x as i32,
                caret_position.y as i32,
            )),
            Size::Physical(PhysicalSize::new(1, line_height.ceil() as u32)),
        );
    }

    pub fn deactivate_keyboard(window: &Window) {
        window.set_ime_allowed(false);
    }
}

impl SystemInterfaceBehaviour for SystemWinit {
    fn set_mouse_cursor(self: &mut SystemInterfaceObject<Self>, cursor: Cursor) {
        Self::set_mouse_cursor(&self.window, cursor);
    }

    fn activate_keyboard(
        self: &mut SystemInterfaceObject<Self>,
        caret_position: Vec2,
        line_height: f32,
    ) where
        Self: Sized,
    {
        Self::activate_keyboard(&self.window, caret_position, line_height);
    }

    fn deactivate_keyboard(self: &mut SystemInterfaceObject<Self>)
    where
        Self: Sized,
    {
        Self::deactivate_keyboard(&self.window);
    }
}
