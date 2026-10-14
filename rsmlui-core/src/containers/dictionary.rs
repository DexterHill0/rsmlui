use rsmlui_sys::dictionary::dictionary_get_variant;

use crate::containers::variant::{TypeVariant, VariantRef};
use crate::object::constness::{Constness, Immut};
use crate::object::foreign::{ForeignHandle, ForeignObject, ObjectPtr};
use crate::object::ownership::{Foreign, ObjectOwnership};

pub mod tags {
    pub enum TagDictionary {}
}

impl ForeignObject for tags::TagDictionary {
    type Object = rsmlui_sys::dictionary::Dictionary;
}

/// Shared handle newtype for both [`Dictionary`] and [`DictionaryRef`].
#[repr(transparent)]
pub struct DictionaryHandle<'a, M: ObjectOwnership<tags::TagDictionary>, C: Constness>(
    ForeignHandle<'a, tags::TagDictionary, M, C>,
);

// pub type Dictionary = DictionaryHandle<'static, Native, Mut>;

/// A borrowed version of [`Dictionary`].
pub type DictionaryRef<'a> = DictionaryHandle<'a, Foreign, Immut>;

impl<'a> DictionaryRef<'a> {
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `Dictionary` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectPtr<tags::TagDictionary>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, Immut>::from_raw(raw) })
    }
}

impl<'a> DictionaryRef<'a> {
    pub fn try_get<T: TypeVariant + ?Sized, K: AsRef<str>>(&self, key: K) -> Option<&'a T> {
        // Safety: `self.raw` is a valid `Dictionary`
        let variant = unsafe { dictionary_get_variant(self.0.as_ptr(), key.as_ref()) };

        if variant.is_null() {
            return None;
        }

        // Safety: `variant` is a valid non-null pointer
        let variant = unsafe { VariantRef::from_raw(variant) };

        variant.try_get_as::<T>()
    }

    pub fn get<T: TypeVariant + ?Sized, K: AsRef<str>>(&self, key: K) -> &'a T {
        self.try_get::<T, K>(key).expect("could not get key")
    }
}
