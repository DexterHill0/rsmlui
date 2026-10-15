#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    struct DictionaryInsert {
        failed: *const Variant,
        inserted: *const Variant,
    }

    #[namespace = "Rml"]
    unsafe extern "C++" {
        type Dictionary;

        type Variant = crate::variant::Variant;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Dictionary.h");

        fn new_dictionary() -> *mut Dictionary;

        /// # Safety
        ///
        /// - `dictionary` must be a valid, non-null pointer returned by [`new_dictionary`].
        unsafe fn dictionary_destructor(dictionary: *mut Dictionary);

        unsafe fn dictionary_insert<'a>(
            dictionary: *mut Dictionary,
            key: &'a str,
            value: *const Variant,
        ) -> DictionaryInsert;

        unsafe fn dictionary_get_variant_const<'a>(
            dictionary: *const Dictionary,
            key: &'a str,
        ) -> *const Variant;

        unsafe fn dictionary_get_variant<'a>(
            dictionary: *mut Dictionary,
            key: &'a str,
        ) -> *mut Variant;
    }
}

pub use ffi::{
    Dictionary, DictionaryInsert, dictionary_destructor, dictionary_get_variant,
    dictionary_get_variant_const, dictionary_insert, new_dictionary,
};
