#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    struct DictionaryInsert {
        failed: *const Variant,
        inserted: *const Variant,
    }

    struct DictionaryIterPair<'a> {
        finished: bool,
        key: &'a str,
        value: *const Variant,
        iter_base: *const c_void,
    }

    struct DictionaryIterSizeHint {
        lower: usize,
        upper: usize,
    }

    #[namespace = "Rml"]
    unsafe extern "C++" {
        type Dictionary;

        type Variant = crate::variant::Variant;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type c_void = crate::utils::c_void;
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

        unsafe fn dictionary_iterator_begin(dictionary: *const Dictionary) -> *const c_void;
        unsafe fn dictionary_iterator_next<'a>(
            dictionary: *const Dictionary,
            iter_base: *const c_void,
        ) -> DictionaryIterPair<'a>;
        unsafe fn dictionary_iterator_count<'a>(dictionary: *const Dictionary) -> usize;
        unsafe fn dictionary_iterator_size_hint<'a>(
            dictionary: *const Dictionary,
            iter_base: *const c_void,
        ) -> DictionaryIterSizeHint;
    }
}

pub use ffi::{
    Dictionary, DictionaryInsert, DictionaryIterPair, DictionaryIterSizeHint,
    dictionary_destructor, dictionary_get_variant, dictionary_get_variant_const, dictionary_insert,
    dictionary_iterator_begin, dictionary_iterator_count, dictionary_iterator_next,
    dictionary_iterator_size_hint, new_dictionary,
};
