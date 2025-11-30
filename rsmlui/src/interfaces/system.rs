use glam::Vec2;
use rsmlui_sys::system_interface::SystemInterfaceExt;
use std::path::PathBuf;

use crate::{core::log::LogLevel, utils::cursor::Cursor};

pub trait SystemInterface {
    fn get_elapsed_time(&self) -> f64;
    fn translate_string(&self, input: String) -> String;
    fn join_path(&self, document_path: PathBuf, path: PathBuf) -> PathBuf;
    fn log_message(&self, level: LogLevel, message: String);
    fn set_mouse_cursor(&self, cursor_name: Cursor);
    fn set_clipboard_text(&self, text: String);
    fn get_clipboard_text(&self) -> String;
    fn activate_keyboard(&self, caret_position: Vec2, line_height: f32);
    fn deactivate_keyboard(&self);
}
