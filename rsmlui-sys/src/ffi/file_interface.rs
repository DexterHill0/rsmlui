use crate::Rml_FileHandle;
use crate::interfaces::bridge::{InterfaceBridge, fat_from_cpp};

/// Dispatch trait called by C++ when it invokes a virtual method on `RustFileInterface`.
///
/// Implemented by the safe crate on its `InterfaceHandle<T, RustFileInterface>` type, which
/// routes each call to the corresponding method on the user's `FileInterface` impl.
///
/// This is an implementation detail and shouldn't be used directly.
#[doc(hidden)]
pub unsafe trait FileInterfaceBridge {
    unsafe fn open(&mut self, path: &str) -> Rml_FileHandle;
    unsafe fn close(&mut self, file: Rml_FileHandle);
    unsafe fn read(&mut self, buffer: *mut u8, size: usize, file: Rml_FileHandle) -> usize;
    unsafe fn seek(&mut self, file: Rml_FileHandle, offset: i64, origin: i32) -> bool;
    unsafe fn tell(&mut self, file: Rml_FileHandle) -> usize;
    unsafe fn length(&mut self, file: Rml_FileHandle) -> usize;
    unsafe fn load_file(&mut self, path: &str, out_data: &mut Vec<u8>) -> bool;
}

unsafe fn rust_open(cpp_this: *mut InterfaceBridge, path: &str) -> Rml_FileHandle {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).open(path) }
}

unsafe fn rust_close(cpp_this: *mut InterfaceBridge, file: Rml_FileHandle) {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).close(file) }
}

unsafe fn rust_read(
    cpp_this: *mut InterfaceBridge,
    buffer: *mut u8,
    size: usize,
    file: Rml_FileHandle,
) -> usize {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).read(buffer, size, file) }
}

unsafe fn rust_seek(
    cpp_this: *mut InterfaceBridge,
    file: Rml_FileHandle,
    offset: i64,
    origin: i32,
) -> bool {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).seek(file, offset, origin) }
}

unsafe fn rust_tell(cpp_this: *mut InterfaceBridge, file: Rml_FileHandle) -> usize {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).tell(file) }
}

unsafe fn rust_length(cpp_this: *mut InterfaceBridge, file: Rml_FileHandle) -> usize {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).length(file) }
}

unsafe fn rust_load_file(
    cpp_this: *mut InterfaceBridge,
    path: &str,
    out_data: &mut Vec<u8>,
) -> bool {
    unsafe { (*fat_from_cpp::<_, dyn FileInterfaceBridge>(cpp_this)).load_file(path, out_data) }
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        // Although unused in this file, it's defined here so the core, backend,
        // and other modules point to the same generated type
        #[cxx_name = "FileInterface"]
        type RmlFileInterface;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type c_void = crate::utils::c_void;
    }

    #[namespace = "rsmlui::interfaces"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type InterfaceBridge = crate::interfaces::bridge::InterfaceBridge;
    }

    #[namespace = "rsmlui::file_interface"]
    unsafe extern "C++" {
        include!("rsmlui/FileInterface.h");

        /// A struct that represents a file interface created in rust.
        ///
        /// This essentially "inherits" from `RmlFileInterface`, but it's its own
        /// struct to be able to hold the necessary data for rust-based interfaces
        /// to work.
        type RustFileInterface;

        /// # Safety
        ///
        /// - `interface_meta` and `interface_data` must be valid pointers constructed from `to_raw_parts`.
        unsafe fn new_rust_file_interface(
            interface_meta: *const c_void,
            interface_data: *mut c_void,
        ) -> *mut RustFileInterface;

        /// # Safety
        ///
        /// - `obj` must be a valid, non-null pointer to a `RustFileInterface`.
        unsafe fn rust_file_interface_destructor(obj: *mut RustFileInterface);

        // Calls the base `Rml::FileInterface` implementation directly, bypassing any override.
        // Only `Length` and `LoadFile` are non-pure-virtual and have a base implementation.
        #[doc(hidden)]
        unsafe fn file_interface_default_length(ptr: *mut RustFileInterface, file: usize) -> usize; // Rml_FileHandle
        #[doc(hidden)]
        unsafe fn file_interface_default_load_file(
            ptr: *mut RustFileInterface,
            path: &str,
            out_data: &mut Vec<u8>,
        ) -> bool;
    }

    extern "Rust" {
        unsafe fn rust_open(cpp_this: *mut InterfaceBridge, path: &str) -> usize; // Rml_FileHandle
        unsafe fn rust_close(cpp_this: *mut InterfaceBridge, file: usize); // Rml_FileHandle
        unsafe fn rust_read(
            cpp_this: *mut InterfaceBridge,
            buffer: *mut u8,
            size: usize,
            file: usize, // Rml_FileHandle
        ) -> usize;
        unsafe fn rust_seek(
            cpp_this: *mut InterfaceBridge,
            file: usize, // Rml_FileHandle
            offset: i64,
            origin: i32,
        ) -> bool;
        unsafe fn rust_tell(cpp_this: *mut InterfaceBridge, file: usize) -> usize; // Rml_FileHandle
        unsafe fn rust_length(cpp_this: *mut InterfaceBridge, file: usize) -> usize; // Rml_FileHandle
        unsafe fn rust_load_file(
            cpp_this: *mut InterfaceBridge,
            path: &str,
            out_data: &mut Vec<u8>,
        ) -> bool;
    }
}

pub use ffi::{
    RmlFileInterface, RustFileInterface, file_interface_default_length,
    file_interface_default_load_file, new_rust_file_interface, rust_file_interface_destructor,
};

pub use super::interfaces::file_default::{
    default_file_interface_destructor, new_default_file_interface,
};
