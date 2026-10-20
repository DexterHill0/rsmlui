use crate::dictionary::Dictionary;
use crate::interfaces::bridge::{InterfaceBridge, fat_from_cpp};

/// Dispatch trait called by C++ when it invokes a virtual method on `RustXmlNodeHandler`.
///
/// Implemented by the safe crate on its `InterfaceObject<T, RustXmlNodeHandler>` type, which
/// routes each call to the corresponding method on the user's `XmlNodeHandlerBehaviour` impl.
///
/// This is an implementation detail and shouldn't be used directly.
#[doc(hidden)]
pub unsafe trait XmlNodeHandlerBridge {
    unsafe fn element_start(
        &mut self,
        parser: *mut XmlParser,
        name: &str,
        attributes: &Dictionary,
    );
    unsafe fn element_end(&mut self, parser: *mut XmlParser, name: &str) -> bool;
    unsafe fn element_data(
        &mut self,
        parser: *mut XmlParser,
        data: &str,
        data_type: XmlDataType,
    ) -> bool;
}

unsafe fn rust_element_start(
    cpp_this: *mut InterfaceBridge,
    parser: *mut XmlParser,
    name: &str,
    attributes: &Dictionary,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn XmlNodeHandlerBridge>(cpp_this))
            .element_start(parser, name, attributes)
    }
}

unsafe fn rust_element_end(
    cpp_this: *mut InterfaceBridge,
    parser: *mut XmlParser,
    name: &str,
) -> bool {
    unsafe { (*fat_from_cpp::<_, dyn XmlNodeHandlerBridge>(cpp_this)).element_end(parser, name) }
}

unsafe fn rust_element_data(
    cpp_this: *mut InterfaceBridge,
    parser: *mut XmlParser,
    data: &str,
    data_type: XmlDataType,
) -> bool {
    unsafe {
        (*fat_from_cpp::<_, dyn XmlNodeHandlerBridge>(cpp_this))
            .element_data(parser, data, data_type)
    }
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        type Dictionary = crate::dictionary::Dictionary;

        // Although unused in this file, it's defined here so the core, backend,
        // and other modules point to the same generated type
        #[cxx_name = "XMLParser"]
        type XmlParser;

        #[cxx_name = "XMLNodeHandler"]
        type RmlXmlNodeHandler;
    }

    #[namespace = "rsmlui::xml"]
    extern "C++" {
        include!("rsmlui/Utils.h");

        type XmlDataType = crate::Rml_XmlDataType;
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

    #[namespace = "rsmlui::xml_node_handler"]
    unsafe extern "C++" {
        include!("rsmlui/XmlNodeHandler.h");

        type RustXmlNodeHandler;

        /// # Safety
        ///
        /// - `rust_meta` and `rust_data` must be valid pointers constructed from `to_raw_parts`.
        unsafe fn new_rust_xml_node_handler(
            rust_meta: *const c_void,
            rust_data: *mut c_void,
        ) -> *mut RustXmlNodeHandler;

        /// # Safety
        ///
        /// - `obj` must be a valid, non-null pointer to a `RustXmlNodeHandler`.
        unsafe fn rust_xml_node_handler_destructor(obj: *mut RustXmlNodeHandler);

        /// # Safety
        ///
        /// - `handler` must be a valid, non-null pointer to a `XMLNodeHandler` that
        ///   outlives RmlUi's shutdown.
        unsafe fn register_xml_node_handler(tag: &str, handler: *mut RmlXmlNodeHandler);
    }

    extern "Rust" {
        unsafe fn rust_element_start(
            cpp_this: *mut InterfaceBridge,
            parser: *mut XmlParser,
            name: &str,
            attributes: &Dictionary,
        );
        unsafe fn rust_element_end(
            cpp_this: *mut InterfaceBridge,
            parser: *mut XmlParser,
            name: &str,
        ) -> bool;
        unsafe fn rust_element_data(
            cpp_this: *mut InterfaceBridge,
            parser: *mut XmlParser,
            data: &str,
            data_type: XmlDataType,
        ) -> bool;
    }
}

pub use ffi::{
    RmlXmlNodeHandler, RustXmlNodeHandler, XmlDataType, XmlParser, new_rust_xml_node_handler,
    register_xml_node_handler, rust_xml_node_handler_destructor,
};
