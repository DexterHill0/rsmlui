pub trait SystemInterfaceExt {
    fn get_elapsed_time(&self) -> f64;

    fn translate_string(&mut self, translated: &mut String, input: &str) -> i32;

    fn join_path(&mut self, translated_path: &mut String, document_path: &str, path: &str);

    fn log_message(&mut self, level: crate::Rml_Log_Type, msg: &str) -> bool;

    fn set_mouse_cursor(&mut self, name: &str);

    fn set_clipboard_text(&mut self, text: &str);
    fn get_clipboard_text(&mut self, out: &mut String);

    fn activate_keyboard(&mut self, caret: crate::Rml_Vector2f, line_height: f32);
    fn deactivate_keyboard(&mut self);
}

#[repr(C)]
pub struct RustInterfaceOpaque {
    _private: [u8; 0],
}

// pub trait IntoSystemInterfacePtr {
//     fn into_ptr(self) -> *mut rsmlui_sys::system_interface::SystemInterface;
// }

impl IntoPtr<SystemInterface> for *mut SystemInterface {
    fn into_ptr(self) -> *mut SystemInterface {
        self
    }
}

impl<T: SystemInterfaceExt + 'static> IntoPtr<SystemInterface> for T {
    fn into_ptr(self) -> *mut SystemInterface {
        let boxed_trait: Box<dyn SystemInterfaceExt> = Box::new(self);

        let raw = Box::into_raw(boxed_trait) as *mut RustInterfaceOpaque;

        let unique = unsafe { rust_system_interface_new(raw) };
        // drops rust's ownership so RmlUi can take ownership and control the lifetime of the interface
        let raw_cpp_ptr = cxx::UniquePtr::into_raw(unique);

        raw_cpp_ptr as *mut SystemInterface
    }
}

unsafe fn ext_get_elapsed_time(opaque_ptr: *mut RustInterfaceOpaque) -> f64 {
    std::panic::catch_unwind(|| {
        let iface = unsafe { &mut *(opaque_ptr as *mut Box<dyn SystemInterfaceExt>) };
        iface.get_elapsed_time()
    })
    .unwrap_or(0.0)
}

unsafe fn ext_drop_interface(opaque_ptr: *mut RustInterfaceOpaque) {
    let boxed = unsafe { Box::from_raw(opaque_ptr as *mut Box<dyn SystemInterfaceExt>) };
    drop(boxed);
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        type SystemInterface;
    }

    #[namespace = "rsmlui::rust_system_interface"]
    unsafe extern "C++" {
        #[doc(hidden)]
        pub(self) type RustSystemInterface = crate::ffi::rust_system_interface::RustSystemInterface;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        #[doc(hidden)]
        pub(self) unsafe fn rust_system_interface_new(
            obj: *mut RustInterfaceOpaque,
        ) -> UniquePtr<RustSystemInterface>;
    }

    extern "Rust" {
        pub(self) type RustInterfaceOpaque;

        pub(self) unsafe fn ext_get_elapsed_time(interface_ptr: *mut RustInterfaceOpaque) -> f64;
        pub(self) unsafe fn ext_drop_interface(interface_ptr: *mut RustInterfaceOpaque);
    }
}

pub use ffi::*;

use crate::utils::IntoPtr;
