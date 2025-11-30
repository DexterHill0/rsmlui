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

// impl<T: SystemInterfaceExt> From<T> for *mut SystemInterface {
//     fn from(value: T) -> Self {
//         let trait_obj: Box<dyn SystemInterfaceExt> = Box::new(value);

//         unsafe { Box::into_raw(Box::new(trait_obj)) as *mut SystemInterface }
//     }
// }

unsafe fn ext_get_elapsed_time(interface_ptr: *mut RustInterfaceOpaque) -> f64 {
    let interface = unsafe { &mut *(interface_ptr as *mut Box<dyn SystemInterfaceExt>) };
    interface.get_elapsed_time()
}

unsafe fn ext_drop_interface(interface_ptr: *mut RustInterfaceOpaque) {
    unsafe { Box::from_raw(interface_ptr as *mut Box<dyn SystemInterfaceExt>) };
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        type SystemInterface;
    }

    #[namespace = "rsmlui::system_interface"]
    unsafe extern "C++" {
        include!("rsmlui/SystemInterface.h");

        #[doc(hidden)]
        pub(self) type RustSystemInterface = cxx::type_id!(RustSystemInterfaceImpl);

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
