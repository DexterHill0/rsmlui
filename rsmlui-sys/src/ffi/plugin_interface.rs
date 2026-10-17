use crate::context::Context;
use crate::element_document::ElementDocument;
use crate::interfaces::bridge::fat_from_cpp;

/// Dispatch trait called by C++ when it invokes a virtual method on `RustPluginInterface`.
///
/// Implemented by the safe crate on its `InterfaceHandle<T, RustPluginInterface>` type, which
/// routes each call to the corresponding method on the user's `PluginInterface` impl.
///
/// This is an implementation detail and shouldn't be used directly.
#[doc(hidden)]
pub unsafe trait PluginInterfaceBridge {
    unsafe fn on_initialize(&mut self);
    unsafe fn on_shutdown(&mut self);
    unsafe fn on_document_open(&mut self, context: *mut Context, document_path: &str);
    unsafe fn on_document_load(&mut self, document: *mut ElementDocument);
    unsafe fn on_document_unload(&mut self, document: *mut ElementDocument);
    unsafe fn on_context_create(&mut self, context: *mut Context);
    unsafe fn on_context_destroy(&mut self, context: *mut Context);
    // unsafe fn on_element_create(&mut self, element: *mut Element);
    // unsafe fn on_element_destoryb(&mut self, element: *mut Element);
    unsafe fn on_data_model_create(&mut self, context: *mut Context, name: &str);
    unsafe fn on_data_model_destroy(&mut self, context: *mut Context, name: &str);
}

unsafe fn rust_on_initialize(cpp_this: *mut RustPluginInterface) {
    unsafe { (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this)).on_initialize() }
}
unsafe fn rust_on_shutdown(cpp_this: *mut RustPluginInterface) {
    unsafe { (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this)).on_shutdown() }
}
unsafe fn rust_on_document_open(
    cpp_this: *mut RustPluginInterface,
    context: *mut Context,
    document_path: &str,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this))
            .on_document_open(context, document_path)
    }
}
unsafe fn rust_on_document_load(
    cpp_this: *mut RustPluginInterface,
    document: *mut ElementDocument,
) {
    unsafe { (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this)).on_document_load(document) }
}
unsafe fn rust_on_document_unload(
    cpp_this: *mut RustPluginInterface,
    document: *mut ElementDocument,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this)).on_document_unload(document)
    }
}
unsafe fn rust_on_context_create(cpp_this: *mut RustPluginInterface, context: *mut Context) {
    unsafe { (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this)).on_context_create(context) }
}
unsafe fn rust_on_context_destroy(cpp_this: *mut RustPluginInterface, context: *mut Context) {
    unsafe { (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this)).on_context_destroy(context) }
}
// unsafe fn rust_on_element_create(element: *mut Element) {}
// unsafe fn rust_on_element_destoryb(element: *mut Element) {}
unsafe fn rust_on_data_model_create(
    cpp_this: *mut RustPluginInterface,
    context: *mut Context,
    name: &str,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this))
            .on_data_model_create(context, name)
    }
}
unsafe fn rust_on_data_model_destroy(
    cpp_this: *mut RustPluginInterface,
    context: *mut Context,
    name: &str,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn PluginInterfaceBridge>(cpp_this))
            .on_data_model_destroy(context, name)
    }
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        // Although unused in this file, it's defined here so the core, backend,
        // and other modules point to the same generated type
        #[cxx_name = "Plugin"]
        type RmlPlugin;

        type Context = crate::context::Context;
        type ElementDocument = crate::element_document::ElementDocument;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type c_void = crate::utils::c_void;
    }

    #[namespace = "rsmlui::plugin"]
    unsafe extern "C++" {
        include!("rsmlui/PluginInterface.h");

        /// A struct that represents a plugin created in rust.
        type RustPluginInterface;

        /// # Safety
        ///
        /// - `interface_meta` and `interface_data` must be valid pointers constructed from `to_raw_parts`.
        unsafe fn new_rust_plugin_interface(
            interface_meta: *const c_void,
            interface_data: *mut c_void,
        ) -> *mut RustPluginInterface;

        /// # Safety
        ///
        /// - `obj` must be a valid, non-null pointer to a `RustPluginInterface`.
        unsafe fn rust_plugin_interface_destructor(obj: *mut RustPluginInterface);
    }

    extern "Rust" {
        unsafe fn rust_on_initialize(cpp_this: *mut RustPluginInterface);
        unsafe fn rust_on_shutdown(cpp_this: *mut RustPluginInterface);
        unsafe fn rust_on_document_open(
            cpp_this: *mut RustPluginInterface,
            context: *mut Context,
            document_path: &str,
        );
        unsafe fn rust_on_document_load(
            cpp_this: *mut RustPluginInterface,
            document: *mut ElementDocument,
        );
        unsafe fn rust_on_document_unload(
            cpp_this: *mut RustPluginInterface,
            document: *mut ElementDocument,
        );
        unsafe fn rust_on_context_create(cpp_this: *mut RustPluginInterface, context: *mut Context);
        unsafe fn rust_on_context_destroy(
            cpp_this: *mut RustPluginInterface,
            context: *mut Context,
        );
        // unsafe fn rust_on_element_create(cpp_this: *mut RustPluginInterface, element: *mut Element);
        // unsafe fn rust_on_element_destoryb(cpp_this: *mut RustPluginInterface, element: *mut Element);
        unsafe fn rust_on_data_model_create(
            cpp_this: *mut RustPluginInterface,
            context: *mut Context,
            name: &str,
        );
        unsafe fn rust_on_data_model_destroy(
            cpp_this: *mut RustPluginInterface,
            context: *mut Context,
            name: &str,
        );
    }
}

pub use ffi::{
    RmlPlugin, RustPluginInterface, new_rust_plugin_interface, rust_plugin_interface_destructor,
};
