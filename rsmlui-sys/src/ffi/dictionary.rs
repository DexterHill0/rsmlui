#[cxx::bridge]
mod ffi {

    #[namespace = "Rml"]
    unsafe extern "C++" {
        type Dictionary;

        type Variant = crate::variant::Variant;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Dictionary.h");

        unsafe fn dictionary_get_variant<'a>(
            dictionary: *const Dictionary,
            key: &'a str,
        ) -> *const Variant;
    }
}

pub use ffi::{Dictionary, dictionary_get_variant};
