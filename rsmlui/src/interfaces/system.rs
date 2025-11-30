use glam::Vec2;
use rsmlui_sys::utils::IntoPtr;
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

pub(crate) trait IntoSystemInterfacePtr {
    fn into_ptr(self) -> *mut rsmlui_sys::system_interface::SystemInterface;
}

impl<T: SystemInterface + 'static> IntoSystemInterfacePtr for T {
    fn into_ptr(self) -> *mut rsmlui_sys::system_interface::SystemInterface {
        let adapter = SystemInterfaceExtAdapter { inner: self };

        adapter.into_ptr()
    }
}

pub(crate) struct SystemInterfaceExtAdapter<T: SystemInterface> {
    pub(crate) inner: T,
}

impl<T: SystemInterface> rsmlui_sys::system_interface::SystemInterfaceExt
    for SystemInterfaceExtAdapter<T>
{
    fn get_elapsed_time(&self) -> f64 {
        todo!()
    }

    fn translate_string(&mut self, translated: &mut String, input: &str) -> i32 {
        todo!()
    }

    fn join_path(&mut self, translated_path: &mut String, document_path: &str, path: &str) {
        todo!()
    }

    fn log_message(&mut self, level: rsmlui_sys::Rml_Log_Type, msg: &str) -> bool {
        todo!()
    }

    fn set_mouse_cursor(&mut self, name: &str) {
        todo!()
    }

    fn set_clipboard_text(&mut self, text: &str) {
        todo!()
    }

    fn get_clipboard_text(&mut self, out: &mut String) {
        todo!()
    }

    fn activate_keyboard(&mut self, caret: rsmlui_sys::Rml_Vector2f, line_height: f32) {
        todo!()
    }

    fn deactivate_keyboard(&mut self) {
        todo!()
    }
}
