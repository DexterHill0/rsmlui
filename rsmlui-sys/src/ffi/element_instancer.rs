use crate::dictionary::Dictionary;
use crate::element::Element;
use crate::interfaces::bridge::{InterfaceBridge, fat_from_cpp};

/// Dispatch trait called by C++ when it invokes a virtual method on `RustElementInstancer`.
///
/// Implemented by the safe crate on its `InterfaceObject<T, RustElementInstancer>` type, which
/// routes each call to the corresponding method on the user's `ElementInstancerBehaviour` impl.
///
/// This is an implementation detail and shouldn't be used directly.
#[doc(hidden)]
pub unsafe trait ElementInstancerBridge {
    unsafe fn instance_element(&mut self, parent: *mut Element, tag: &str, attributes: &Dictionary);
    unsafe fn release_element(&mut self, element: *mut Element);
}

unsafe fn rust_instance_element(
    cpp_this: *mut InterfaceBridge,
    parent: *mut Element,
    tag: &str,
    attributes: &Dictionary,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn ElementInstancerBridge>(cpp_this))
            .instance_element(parent, tag, attributes)
    }
}

unsafe fn rust_release_element(cpp_this: *mut InterfaceBridge, element: *mut Element) {
    unsafe { (*fat_from_cpp::<_, dyn ElementInstancerBridge>(cpp_this)).release_element(element) }
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        type Element = crate::element::Element;
        type Dictionary = crate::dictionary::Dictionary;

        #[cxx_name = "ElementInstancer"]
        type RmlElementInstancer;
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

    #[namespace = "rsmlui::element_instancer"]
    unsafe extern "C++" {
        include!("rsmlui/ElementInstancer.h");

        type RustElementInstancer;

        /// # Safety
        ///
        /// - `rust_meta` and `rust_data` must be valid pointers constructed from `to_raw_parts`.
        unsafe fn new_rust_element_instancer(
            rust_meta: *const c_void,
            rust_data: *mut c_void,
        ) -> *mut RustElementInstancer;

        /// # Safety
        ///
        /// - `obj` must be a valid, non-null pointer to a `RustElementInstancer`.
        unsafe fn rust_element_instancer_destructor(obj: *mut RustElementInstancer);
    }

    extern "Rust" {
        unsafe fn rust_instance_element(
            cpp_this: *mut InterfaceBridge,
            parent: *mut Element,
            tag: &str,
            attributes: &Dictionary,
        );
        unsafe fn rust_release_element(cpp_this: *mut InterfaceBridge, element: *mut Element);
    }
}

pub use ffi::{
    RmlElementInstancer, RustElementInstancer, new_rust_element_instancer,
    rust_element_instancer_destructor,
};
