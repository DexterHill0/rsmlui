use std::fmt::Debug;

use rsmlui_sys::dictionary::{self as sys};

use crate::_private::{not_send_sync, not_unwind_safe};
use crate::containers::variant::{TypeVariant, Variant, VariantRef, VariantRefMut};
use crate::object::constness::{Const, Constness, NonConst};
use crate::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject, ObjectPtr};
use crate::object::ownership::{Foreign, Native, ObjectOwnership};

pub mod tags {
    pub enum TagDictionary {}
}

impl ForeignObject for tags::TagDictionary {
    type Object = rsmlui_sys::dictionary::Dictionary;
}

unsafe impl ForeignDestroy for tags::TagDictionary {
    unsafe fn destroy(raw: *mut Self::Object) {
        // Safety: raw is a valid, non-null pointer
        unsafe { sys::dictionary_destructor(raw) }
    }
}

/// Shared handle newtype for both [`Dictionary`] and [`DictionaryRef`].
#[repr(transparent)]
pub struct DictionaryHandle<'a, M: ObjectOwnership<tags::TagDictionary>, C: Constness>(
    ForeignHandle<'a, tags::TagDictionary, M, C>,
);

/// Holds [`String`]: [`Variant`] key-value pairs.
pub type Dictionary = DictionaryHandle<'static, Native, NonConst>;

/// A immutably borrowed version of [`Dictionary`].
pub type DictionaryRef<'a> = DictionaryHandle<'a, Foreign, Const>;

/// A mutably borrowed version of [`Dictionary`].
pub type DictionaryRefMut<'a> = DictionaryHandle<'a, Foreign, NonConst>;

pub struct Iter<'a> {
    handle: DictionaryRef<'a>,
    iter_base: *const (),
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, VariantRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let item =
            unsafe { sys::dictionary_iterator_next(self.handle.0.as_ptr(), self.iter_base.cast()) };

        if item.finished {
            debug_assert!(item.key.is_empty());
            debug_assert!(item.value.is_null());

            return None;
        }

        self.iter_base = item.iter_base.cast();

        Some((item.key, unsafe { VariantRef::from_raw(item.value) }))
    }

    fn count(self) -> usize {
        unsafe { sys::dictionary_iterator_count(self.handle.0.as_ptr()) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = unsafe {
            sys::dictionary_iterator_size_hint(self.handle.0.as_ptr(), self.iter_base.cast())
        };

        (hint.lower, Some(hint.upper))
    }
}

not_send_sync!(['a] Iter['a]);
not_unwind_safe!(['a] Iter['a]);

impl Default for Dictionary {
    fn default() -> Self {
        // Safety: the allocation is a brand new object and not a reference to
        // any existing data.
        Self(unsafe { ForeignHandle::from_raw(sys::new_dictionary()) })
    }
}

macro_rules! dictionary_methods {
    ($lt:lifetime) => {
        pub fn get_variant<K: AsRef<str>>(&self, key: K) -> VariantRef<$lt> {
            let key = key.as_ref();

            self.try_get_variant(key)
                .unwrap_or_else(|| panic!("no such key `{key}`"))
        }

        pub fn try_get_variant<K: AsRef<str>>(&self, key: K) -> Option<VariantRef<$lt>> {
            let key = key.as_ref();

            let variant = unsafe { sys::dictionary_get_variant_const(self.0.as_ptr(), key) };

            if variant.is_null() {
                return None;
            }

            // Safety: `variant` is a valid non-null pointer
            Some(unsafe { VariantRef::from_raw(variant) })
        }

        pub fn try_get<K: AsRef<str>, T: TypeVariant + ?Sized>(
            &self,
            key: K,
        ) -> Option<<T as ToOwned>::Owned> {
            self.try_get_variant(key)?.try_get_as::<T>()
        }

        pub fn get<K: AsRef<str>, T: TypeVariant + ?Sized>(&self, key: K) -> <T as ToOwned>::Owned {
            self.get_variant(key).get_as::<T>()
        }

        /// # Safety
        ///
        /// The iterator returns borrows into foreign memory. Therefore:
        ///
        /// - The dictionary MUST NOT be mutated while an iterator is active.
        ///
        /// This includes anything that could move or destroy it.
        pub unsafe fn iter(&self) -> Iter<$lt> {
            let begin = unsafe { sys::dictionary_iterator_begin(self.0.as_ptr()) };

            Iter {
                handle: unsafe { DictionaryRef::from_raw(self.0.as_ptr()) },
                iter_base: begin.cast(),
            }
        }
    };
}

// Only native dictionaries can be created
impl Dictionary {
    /// Creates a new [`Dictionary`].
    ///
    /// A dictionary contains only unique keys, and a key's value cannot be
    /// overwritten without first removing.
    pub fn new() -> Self {
        Self::default()
    }
}

// As per `ForeignHandle` docs, we can only return objects with `'a` lifetime
// when `M = Foreign` and `C = Const`, as that allows the object to outlive
// the handle ref
impl<'a> DictionaryRef<'a> {
    dictionary_methods!('a);

    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `Dictionary` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectPtr<tags::TagDictionary>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, Const>::from_raw(raw) })
    }
}

// Any other combination must tie the return data to `'self`
impl<'a> DictionaryHandle<'a, Native, Const> {
    dictionary_methods!('_);
}

impl<'a, M: ObjectOwnership<tags::TagDictionary>> DictionaryHandle<'a, M, NonConst> {
    dictionary_methods!('_);

    /// Inserts a [`Variant`] into the dictionary.
    ///
    /// If the key did not exist, [`Result::Ok`] is returned with a [`VariantRef`]
    /// to the inserted value.
    ///
    /// If the key did exist, [`Result::Err`] is returned with a [`VariantRef`]
    /// to value that prevented insertion.
    pub fn try_insert<K: AsRef<str>>(
        &'_ mut self,
        key: K,
        value: Variant,
    ) -> Result<VariantRef<'_>, VariantRef<'_>> {
        let variant_raw = unsafe { value.as_ptr() };

        // Safety: `self` is a valid mutable dictionary, variant_raw is a valid `Variant`
        let result =
            unsafe { sys::dictionary_insert(self.0.as_mut_ptr(), key.as_ref(), variant_raw) };

        if !result.inserted.is_null() {
            debug_assert!(
                result.failed.is_null(),
                "value inserted but failed is not null"
            );

            // Safety: the insertion was successful, so `result.inserted` will be a valid
            // pointer to the `Variant` that was just inserted.
            // `result.inserted` != `variant_raw` because the variant is copied into the dict,
            // though they are the same value.
            return Ok(unsafe { VariantRef::from_raw(result.inserted) });
        }

        debug_assert!(
            result.inserted.is_null(),
            "value failed to insert but inserted is not null"
        );

        // Safety: the insertion was unsuccessful, so `result.failed` will be a valid
        // pointer to the `Variant` already in the dictionary
        Err(unsafe { VariantRef::from_raw(result.failed) })
    }

    pub fn insert<K: AsRef<str>>(&'_ mut self, key: K, value: Variant) -> VariantRef<'_> {
        let key = key.as_ref();

        self.try_insert(key, value)
            .unwrap_or_else(|_| panic!("key `{}` already exists", key))
    }

    pub fn try_get_variant_mut<K: AsRef<str>>(&mut self, key: K) -> Option<VariantRefMut<'_>> {
        let variant = unsafe { sys::dictionary_get_variant(self.0.as_mut_ptr(), key.as_ref()) };

        if variant.is_null() {
            return None;
        }

        // Safety: `variant` is a valid non-null pointer
        Some(unsafe { VariantRefMut::from_raw(variant) })
    }

    pub fn get_variant_mut<K: AsRef<str>>(&mut self, key: K) -> VariantRefMut<'_> {
        let key = key.as_ref();

        self.try_get_variant_mut(key)
            .unwrap_or_else(|| panic!("no such key `{key}`"))
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;

    impl<'a> Debug for DictionaryRef<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Safety: the iterator is consumed in one go by`entries`.
            // No user code can modify the dictionary while inside this function, so
            // the observed references cannot change.
            f.debug_map().entries(unsafe { self.iter() }).finish()
        }
    }

    impl<'a, M: ObjectOwnership<tags::TagDictionary>> Debug for DictionaryHandle<'a, M, NonConst> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries(unsafe { self.iter() }).finish()
        }
    }

    impl<'a> Debug for DictionaryHandle<'a, Native, Const> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries(unsafe { self.iter() }).finish()
        }
    }
}

#[cfg(not(feature = "printing"))]
impl<'a, M: ObjectOwnership<tags::TagDictionary>, C: Constness> Debug
    for DictionaryHandle<'a, M, C>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ <foreign> }}")
    }
}
