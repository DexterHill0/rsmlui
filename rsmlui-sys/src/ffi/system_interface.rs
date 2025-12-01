use std::cell::UnsafeCell;
use std::collections::btree_set::Union;

use crate::interfaces::{self, InterfaceOpaque, InterfaceOpaquePtr, ThinInterface};

pub trait RawSystemInterface {
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

unsafe fn from_opaque<'a>(opaque_ptr: InterfaceOpaquePtr) -> &'a mut dyn RawSystemInterface {
    debug_assert!(!opaque_ptr.is_null());

    let stored = opaque_ptr as *mut ThinInterface<*mut dyn RawSystemInterface>;

    unsafe { &mut **(*stored).get() }
}

unsafe fn rust_get_elapsed_time(opaque_ptr: InterfaceOpaquePtr) -> f64 {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe { iface.get_elapsed_time() }
}

unsafe fn rust_translate_string(opaque_ptr: InterfaceOpaquePtr, input: &str) -> String {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe { iface.translate_string(input) }
}

unsafe fn rust_join_path(
    opaque_ptr: InterfaceOpaquePtr,
    document_path: &str,
    path: &str,
) -> String {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe { iface.join_path(document_path, path) }
}

unsafe fn rust_log_message(
    opaque_ptr: InterfaceOpaquePtr,
    level: crate::Rml_Log_Type,
    message: &str,
) -> bool {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe { iface.log_message(level, message) }
}

unsafe fn rust_set_mouse_cursor(opaque_ptr: InterfaceOpaquePtr, name: &str) {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe {
        iface.set_mouse_cursor(name);
    };
}

unsafe fn rust_set_clipboard_text(opaque_ptr: InterfaceOpaquePtr, text: &str) {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe {
        iface.set_clipboard_text(text);
    };
}

unsafe fn rust_get_clipboard_text(opaque_ptr: InterfaceOpaquePtr) -> String {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe { iface.get_clipboard_text() }
}

unsafe fn rust_activate_keyboard(
    opaque_ptr: InterfaceOpaquePtr,
    caret_position: crate::Rml_Vector2f,
    line_height: f32,
) {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe {
        iface.activate_keyboard(caret_position, line_height);
    };
}

unsafe fn rust_deactivate_keyboard(opaque_ptr: InterfaceOpaquePtr) {
    let iface = unsafe { from_opaque(opaque_ptr) };

    unsafe {
        iface.deactivate_keyboard();
    };
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "SystemInterface"]
        type RmlSystemInterface;

        type Vector2f = crate::Rml_Vector2f;
    }

    #[namespace = "rsmlui::log"]
    extern "C++" {
        include!("rsmlui/Utils.h");

        type RmlLogType = crate::Rml_Log_Type;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        unsafe fn new_rust_system_interface(obj: *mut InterfaceOpaque) -> *mut RmlSystemInterface;

        unsafe fn rust_system_interface_destructor(obj: *mut RmlSystemInterface);

        // default methods that call the base `Rml::SystemInterface` implementation
        #[doc(hidden)]
        unsafe fn system_interface_get_elapsed_time(opaque_ptr: *mut RmlSystemInterface) -> f64;
        #[doc(hidden)]
        unsafe fn system_interface_translate_string(
            opaque_ptr: *mut RmlSystemInterface,
            input: &str,
        ) -> String;
        #[doc(hidden)]
        unsafe fn system_interface_join_path(
            opaque_ptr: *mut RmlSystemInterface,
            document_path: &str,
            path: &str,
        ) -> String;
        #[doc(hidden)]
        unsafe fn system_interface_log_message(
            opaque_ptr: *mut RmlSystemInterface,
            level: RmlLogType,
            message: &str,
        ) -> bool;
        #[doc(hidden)]
        unsafe fn system_interface_set_mouse_cursor(
            opaque_ptr: *mut RmlSystemInterface,
            name: &str,
        );
        #[doc(hidden)]
        unsafe fn system_interface_set_clipboard_text(
            opaque_ptr: *mut RmlSystemInterface,
            text: &str,
        );
        #[doc(hidden)]
        unsafe fn system_interface_get_clipboard_text(
            opaque_ptr: *mut RmlSystemInterface,
        ) -> String;
        #[doc(hidden)]
        unsafe fn system_interface_activate_keyboard(
            opaque_ptr: *mut RmlSystemInterface,
            caret_position: Vector2f,
            line_height: f32,
        );
        #[doc(hidden)]
        unsafe fn system_interface_deactivate_keyboard(opaque_ptr: *mut RmlSystemInterface);
    }

    extern "Rust" {
        type InterfaceOpaque;

        // these methods are called by the `RustSystemInterface` class in C++
        // this class implements `SystemInterface` so it acts like a system interface, but these rust methods
        // to run the trait methods
        unsafe fn rust_get_elapsed_time(opaque_ptr: *mut InterfaceOpaque) -> f64;
        unsafe fn rust_translate_string(opaque_ptr: *mut InterfaceOpaque, input: &str) -> String;
        unsafe fn rust_join_path(
            opaque_ptr: *mut InterfaceOpaque,
            document_path: &str,
            path: &str,
        ) -> String;
        unsafe fn rust_log_message(
            opaque_ptr: *mut InterfaceOpaque,
            level: RmlLogType,
            message: &str,
        ) -> bool;
        unsafe fn rust_set_mouse_cursor(opaque_ptr: *mut InterfaceOpaque, name: &str);
        unsafe fn rust_set_clipboard_text(opaque_ptr: *mut InterfaceOpaque, text: &str);
        unsafe fn rust_get_clipboard_text(opaque_ptr: *mut InterfaceOpaque) -> String;
        unsafe fn rust_activate_keyboard(
            opaque_ptr: *mut InterfaceOpaque,
            caret_position: Vector2f,
            line_height: f32,
        );
        unsafe fn rust_deactivate_keyboard(opaque_ptr: *mut InterfaceOpaque);
    }
}

pub use ffi::{
    RmlSystemInterface, new_rust_system_interface, rust_system_interface_destructor,
    system_interface_activate_keyboard, system_interface_deactivate_keyboard,
    system_interface_get_clipboard_text, system_interface_get_elapsed_time,
    system_interface_join_path, system_interface_log_message, system_interface_set_clipboard_text,
    system_interface_set_mouse_cursor, system_interface_translate_string,
};
