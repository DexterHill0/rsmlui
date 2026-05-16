use std::marker::PhantomData;

use rsmlui_sys::dictionary::dictionary_get_variant;

use crate::containers::variant::{TypeVariant, Variant};
use crate::utils::raw::{Ptr, Raw};

pub struct Dictionary<'a> {
    pub(crate) raw: Ptr<Self>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Raw for Dictionary<'a> {
    type Ptr = *mut rsmlui_sys::dictionary::Dictionary;

    fn raw(&self) -> Self::Ptr {
        self.raw
    }
}

impl<'a> Dictionary<'a> {
    pub fn try_get<T: TypeVariant, K: AsRef<str>>(&self, key: K) -> Option<&'a T> {
        // Safety: `self.raw` is a valid `Dictionary`
        let variant = unsafe { dictionary_get_variant(self.raw(), key.as_ref()) };

        if variant.is_null() {
            return None;
        }

        // Safety: `variant` is a valid non-null pointer
        let variant = unsafe { Variant::from_raw(variant) };

        variant.try_get_as::<T>()
    }

    pub fn get<T: TypeVariant, K: AsRef<str>>(&self, key: K) -> &'a T {
        self.try_get::<T, K>(key).expect("could not get key")
    }
}
