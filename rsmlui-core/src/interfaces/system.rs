use std::mem::transmute;

use glam::Vec2;
use rsmlui_macros::rmldoc;
use rsmlui_sys::core;
use rsmlui_sys::interfaces::Opaque;
use rsmlui_sys::system_interface::{
    RmlSystemInterface, RustSystemInterface, SystemInterfaceBridge, new_rust_system_interface,
    rust_system_interface_destructor, system_interface_default_activate_keyboard,
    system_interface_default_deactivate_keyboard, system_interface_default_get_clipboard_text,
    system_interface_default_get_elapsed_time, system_interface_default_join_path,
    system_interface_default_log_message, system_interface_default_set_clipboard_text,
    system_interface_default_set_mouse_cursor, system_interface_default_translate_string,
};
use sealed::sealed;

use crate::core::log::LogLevel;
use crate::interfaces::{InterfaceHandle, IntoRawInterface, OwnedInterface, RawInterface};
use crate::types::cursor::Cursor;
use crate::utils::conversions::{FromSys, IntoSys};

/// The receiver type for all [`SystemInterface`] methods.
pub type SystemInterfaceHandle<T> = InterfaceHandle<T, RustSystemInterface>;

/// Implement this trait to create a custom [`Rml::SystemInterface`] for RmlUi.
///
/// Every method has a default implementation that forwards to the C++ base class behaviour.
///
/// Methods receive `self: &mut SystemInterfaceHandle<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
/// [`SystemInterfaceHandle`] implements [`Deref`] and [`DerefMut`] to give access to the
/// underlying user data.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn SystemInterface` trait object. Dispatch must always
/// go through the [`InterfaceHandle`].
///
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
/// [`Rml::SystemInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/system.html
#[rmldoc(file = "api_Rml-SystemInterface.md", name = "Rml::SystemInterface")]
pub trait SystemInterface {
    #[rmldoc(name = "Rml::SystemInterface::GetElapsedTime")]
    fn get_elapsed_time(self: &mut SystemInterfaceHandle<Self>) -> f64
    where
        Self: Sized,
    {
        unsafe { system_interface_default_get_elapsed_time(self.bridge_ptr()) }
    }

    #[rmldoc(name = "Rml::SystemInterface::TranslateString")]
    fn translate_string(self: &mut SystemInterfaceHandle<Self>, input: &str) -> String
    where
        Self: Sized,
    {
        unsafe { system_interface_default_translate_string(self.bridge_ptr(), input) }
    }

    #[rmldoc(name = "Rml::SystemInterface::JoinPath")]
    fn join_path(
        self: &mut SystemInterfaceHandle<Self>,
        document_path: std::path::PathBuf,
        path: std::path::PathBuf,
    ) -> std::path::PathBuf
    where
        Self: Sized,
    {
        unsafe {
            system_interface_default_join_path(
                self.bridge_ptr(),
                document_path.to_str().unwrap(),
                path.to_str().unwrap(),
            )
            .into()
        }
    }

    #[rmldoc(name = "Rml::SystemInterface::LogMessage")]
    fn log_message(self: &mut SystemInterfaceHandle<Self>, level: LogLevel, message: &str) -> bool
    where
        Self: Sized,
    {
        unsafe { system_interface_default_log_message(self.bridge_ptr(), level, message) }
    }

    #[rmldoc(name = "Rml::SystemInterface::SetMouseCursor")]
    fn set_mouse_cursor(self: &mut SystemInterfaceHandle<Self>, cursor: Cursor)
    where
        Self: Sized,
    {
        let name: String = cursor.into();
        unsafe { system_interface_default_set_mouse_cursor(self.bridge_ptr(), &name) }
    }

    #[rmldoc(name = "Rml::SystemInterface::SetClipboardText")]
    fn set_clipboard_text(self: &mut SystemInterfaceHandle<Self>, text: &str)
    where
        Self: Sized,
    {
        unsafe { system_interface_default_set_clipboard_text(self.bridge_ptr(), text) }
    }

    #[rmldoc(name = "Rml::SystemInterface::GetClipboardText")]
    fn get_clipboard_text(self: &mut SystemInterfaceHandle<Self>) -> String
    where
        Self: Sized,
    {
        unsafe { system_interface_default_get_clipboard_text(self.bridge_ptr()) }
    }

    #[rmldoc(name = "Rml::SystemInterface::ActivateKeyboard")]
    fn activate_keyboard(
        self: &mut SystemInterfaceHandle<Self>,
        caret_position: Vec2,
        line_height: f32,
    ) where
        Self: Sized,
    {
        unsafe {
            system_interface_default_activate_keyboard(
                self.bridge_ptr(),
                caret_position.into_sys(),
                line_height,
            )
        }
    }

    #[rmldoc(name = "Rml::SystemInterface::DeactivateKeyboard")]
    fn deactivate_keyboard(self: &mut SystemInterfaceHandle<Self>)
    where
        Self: Sized,
    {
        unsafe { system_interface_default_deactivate_keyboard(self.bridge_ptr()) }
    }
}

// The sys crate uses `SystemInterfaceBridge` as the dispatch from C++ to Rust.
// This impl forwards each call to the user's `SystemInterface` implementation.
unsafe impl<T: SystemInterface> SystemInterfaceBridge for SystemInterfaceHandle<T> {
    #[inline]
    unsafe fn get_elapsed_time(&mut self) -> f64 {
        T::get_elapsed_time(self)
    }

    #[inline]
    unsafe fn translate_string(&mut self, input: &str) -> String {
        T::translate_string(self, input)
    }

    #[inline]
    unsafe fn join_path(&mut self, document_path: &str, path: &str) -> String {
        T::join_path(self, document_path.into(), path.into())
            .to_str()
            .unwrap()
            .to_string()
    }

    #[inline]
    unsafe fn log_message(&mut self, level: rsmlui_sys::Rml_Log_Type, message: &str) -> bool {
        T::log_message(self, level, message)
    }

    #[inline]
    unsafe fn set_mouse_cursor(&mut self, name: &str) {
        T::set_mouse_cursor(self, name.into())
    }

    #[inline]
    unsafe fn set_clipboard_text(&mut self, text: &str) {
        T::set_clipboard_text(self, text)
    }

    #[inline]
    unsafe fn get_clipboard_text(&mut self) -> String {
        T::get_clipboard_text(self)
    }

    #[inline]
    unsafe fn activate_keyboard(
        &mut self,
        caret_position: rsmlui_sys::Rml_Vector2f,
        line_height: f32,
    ) {
        T::activate_keyboard(self, Vec2::from_sys(caret_position), line_height)
    }

    #[inline]
    unsafe fn deactivate_keyboard(&mut self) {
        T::deactivate_keyboard(self)
    }
}

#[sealed]
impl<T: SystemInterface> super::OwnedInterfaceHandle for T {
    type BridgeObj = RustSystemInterface;

    fn init_bridge(handle: &mut SystemInterfaceHandle<T>) {
        // The fat pointer data component is the address of the heap-allocated InterfaceHandle.
        // That address is stable for the lifetime of the OwnedInterface.
        let fat_ptr: *mut dyn SystemInterfaceBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_system_interface(meta_raw as *const Opaque, data as *mut Opaque) };

        handle.bridge = cpp;
    }

    unsafe fn destroy(handle: &mut SystemInterfaceHandle<T>) {
        unsafe { rust_system_interface_destructor(handle.bridge_ptr()) }
    }

    fn assert_not_registered(handle: &InterfaceHandle<Self, Self::BridgeObj>) {
        let current_interface_ptr = core::get_system_interface();

        let self_ptr = unsafe { handle.bridge_ptr() }.cast();

        assert_ne!(
            current_interface_ptr, self_ptr,
            "OwnedInterface dropped while still registered as RmlUI's system interface"
        );
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl<T: SystemInterface> IntoRawInterface<RmlSystemInterface> for &OwnedInterface<T> {
    fn into_raw(self) -> RawInterface<RmlSystemInterface> {
        // `RustSystemInterface` is a subclass of `RmlSystemInterface` so the cast is valid.
        RawInterface::new(self.as_sys_ptr().cast())
    }
}
