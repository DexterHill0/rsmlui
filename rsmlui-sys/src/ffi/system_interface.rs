use std::mem::{offset_of, transmute};
use std::ptr::{self, DynMetadata};

use crate::interfaces::{InterfaceBridgeLayout, Opaque};
use crate::{Layouts_SystemInterfaceLayoutGuard, const_assert_eq};

// Asserts that the layout of the interface bridge matches the layout of the
// `RustSystemInterface` struct in C++.
const _: () = {
    const_assert_eq!(
        offset_of!(Layouts_SystemInterfaceLayoutGuard, _base),
        offset_of!(InterfaceBridgeLayout, cpp_vtable)
    );
    const_assert_eq!(
        offset_of!(Layouts_SystemInterfaceLayoutGuard, rust_meta),
        offset_of!(InterfaceBridgeLayout, rust_meta)
    );
    const_assert_eq!(
        offset_of!(Layouts_SystemInterfaceLayoutGuard, rust_data),
        offset_of!(InterfaceBridgeLayout, rust_data)
    );
};

/// Dispatch trait called by C++ when it invokes a virtual method on `RustSystemInterface`.
///
/// Implemented by the safe crate on its `InterfaceHandle<T, RustSystemInterface>` type, which
/// routes each call to the corresponding method on the user's `SystemInterface` impl.
///
/// This is an implementation detail and shouldn't be used directly.
#[doc(hidden)]
pub unsafe trait SystemInterfaceBridge {
    unsafe fn get_elapsed_time(&mut self) -> f64;
    unsafe fn translate_string(&mut self, input: &str) -> String;
    unsafe fn join_path(&mut self, document_path: &str, path: &str) -> String;
    unsafe fn log_message(&mut self, level: crate::Rml_Log_Type, message: &str) -> bool;
    unsafe fn set_mouse_cursor(&mut self, name: &str);
    unsafe fn set_clipboard_text(&mut self, text: &str);
    unsafe fn get_clipboard_text(&mut self) -> String;
    unsafe fn activate_keyboard(&mut self, caret_position: crate::Rml_Vector2f, line_height: f32);
    unsafe fn deactivate_keyboard(&mut self);
}

/// Reconstructs the fat `dyn SystemInterfaceBridge` pointer from the C++ `this`, which has the
/// same memory layout as `InterfaceBridgeLayout`.
///
/// # Safety
///
/// - Callers must ensure `cpp_this` is a valid, non-null `InterfaceBridgeLayout` that was
///   initialised in C++.
#[inline]
unsafe fn fat_from_cpp(cpp_this: *mut RustSystemInterface) -> *mut dyn SystemInterfaceBridge {
    let bridge = unsafe { &*(cpp_this as *const InterfaceBridgeLayout) };
    let meta: DynMetadata<dyn SystemInterfaceBridge> = unsafe { transmute(bridge.rust_meta) };

    ptr::from_raw_parts_mut(bridge.rust_data, meta)
}

unsafe fn rust_get_elapsed_time(cpp_this: *mut RustSystemInterface) -> f64 {
    unsafe { (*fat_from_cpp(cpp_this)).get_elapsed_time() }
}

unsafe fn rust_translate_string(cpp_this: *mut RustSystemInterface, input: &str) -> String {
    unsafe { (*fat_from_cpp(cpp_this)).translate_string(input) }
}

unsafe fn rust_join_path(
    cpp_this: *mut RustSystemInterface,
    document_path: &str,
    path: &str,
) -> String {
    unsafe { (*fat_from_cpp(cpp_this)).join_path(document_path, path) }
}

unsafe fn rust_log_message(
    cpp_this: *mut RustSystemInterface,
    level: crate::Rml_Log_Type,
    message: &str,
) -> bool {
    unsafe { (*fat_from_cpp(cpp_this)).log_message(level, message) }
}

unsafe fn rust_set_mouse_cursor(cpp_this: *mut RustSystemInterface, name: &str) {
    unsafe { (*fat_from_cpp(cpp_this)).set_mouse_cursor(name) }
}

unsafe fn rust_set_clipboard_text(cpp_this: *mut RustSystemInterface, text: &str) {
    unsafe { (*fat_from_cpp(cpp_this)).set_clipboard_text(text) }
}

unsafe fn rust_get_clipboard_text(cpp_this: *mut RustSystemInterface) -> String {
    unsafe { (*fat_from_cpp(cpp_this)).get_clipboard_text() }
}

unsafe fn rust_activate_keyboard(
    cpp_this: *mut RustSystemInterface,
    caret_position: crate::Rml_Vector2f,
    line_height: f32,
) {
    unsafe { (*fat_from_cpp(cpp_this)).activate_keyboard(caret_position, line_height) }
}

unsafe fn rust_deactivate_keyboard(cpp_this: *mut RustSystemInterface) {
    unsafe { (*fat_from_cpp(cpp_this)).deactivate_keyboard() }
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        type Vector2f = crate::Rml_Vector2f;

        // Although unused in this file, it's defined here so the core, backend,
        // and other modules point to the same generated type
        #[cxx_name = "SystemInterface"]
        type RmlSystemInterface;
    }

    #[namespace = "rsmlui::log"]
    extern "C++" {
        include!("rsmlui/Utils.h");

        type RmlLogType = crate::Rml_Log_Type;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        type RustSystemInterface;

        /// # Safety
        ///
        /// - `interface_meta` and `interface_data` must be valid pointers constructed from `to_raw_parts`.
        unsafe fn new_rust_system_interface(
            interface_meta: *const Opaque,
            interface_data: *mut Opaque,
        ) -> *mut RustSystemInterface;

        /// # Safety
        ///
        /// - `obj` must be a valid, non-null pointer to a `RustSystemInterface`.
        unsafe fn rust_system_interface_destructor(obj: *mut RustSystemInterface);

        // Calls the base `Rml::SystemInterface` implementation directly, bypassing any override.
        // Used by the default method implementations on the safe crate's `SystemInterface` trait.
        #[doc(hidden)]
        unsafe fn system_interface_default_get_elapsed_time(ptr: *mut RustSystemInterface) -> f64;
        #[doc(hidden)]
        unsafe fn system_interface_default_translate_string(
            ptr: *mut RustSystemInterface,
            input: &str,
        ) -> String;
        #[doc(hidden)]
        unsafe fn system_interface_default_join_path(
            ptr: *mut RustSystemInterface,
            document_path: &str,
            path: &str,
        ) -> String;
        #[doc(hidden)]
        unsafe fn system_interface_default_log_message(
            ptr: *mut RustSystemInterface,
            level: RmlLogType,
            message: &str,
        ) -> bool;
        #[doc(hidden)]
        unsafe fn system_interface_default_set_mouse_cursor(
            ptr: *mut RustSystemInterface,
            name: &str,
        );
        #[doc(hidden)]
        unsafe fn system_interface_default_set_clipboard_text(
            ptr: *mut RustSystemInterface,
            text: &str,
        );
        #[doc(hidden)]
        unsafe fn system_interface_default_get_clipboard_text(
            ptr: *mut RustSystemInterface,
        ) -> String;
        #[doc(hidden)]
        unsafe fn system_interface_default_activate_keyboard(
            ptr: *mut RustSystemInterface,
            caret_position: Vector2f,
            line_height: f32,
        );
        #[doc(hidden)]
        unsafe fn system_interface_default_deactivate_keyboard(ptr: *mut RustSystemInterface);
    }

    extern "Rust" {
        type Opaque;

        unsafe fn rust_get_elapsed_time(cpp_this: *mut RustSystemInterface) -> f64;
        unsafe fn rust_translate_string(cpp_this: *mut RustSystemInterface, input: &str) -> String;
        unsafe fn rust_join_path(
            cpp_this: *mut RustSystemInterface,
            document_path: &str,
            path: &str,
        ) -> String;
        unsafe fn rust_log_message(
            cpp_this: *mut RustSystemInterface,
            level: RmlLogType,
            message: &str,
        ) -> bool;
        unsafe fn rust_set_mouse_cursor(cpp_this: *mut RustSystemInterface, name: &str);
        unsafe fn rust_set_clipboard_text(cpp_this: *mut RustSystemInterface, text: &str);
        unsafe fn rust_get_clipboard_text(cpp_this: *mut RustSystemInterface) -> String;
        unsafe fn rust_activate_keyboard(
            cpp_this: *mut RustSystemInterface,
            caret_position: Vector2f,
            line_height: f32,
        );
        unsafe fn rust_deactivate_keyboard(cpp_this: *mut RustSystemInterface);
    }
}

pub use ffi::{
    RmlSystemInterface, RustSystemInterface, new_rust_system_interface,
    rust_system_interface_destructor, system_interface_default_activate_keyboard,
    system_interface_default_deactivate_keyboard, system_interface_default_get_clipboard_text,
    system_interface_default_get_elapsed_time, system_interface_default_join_path,
    system_interface_default_log_message, system_interface_default_set_clipboard_text,
    system_interface_default_set_mouse_cursor, system_interface_default_translate_string,
};
