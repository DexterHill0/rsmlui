use std::marker::PhantomData;

use glam::{Vec2, Vec3, Vec4};
use rsmlui_macros::rmldoc;
use rsmlui_sys::variant::{self as sys};
use sealed::sealed;

use crate::FromSys;
use crate::utils::raw::{Ptr, Raw};

#[sealed]
pub trait TypeVariant {
    const VARIANT_TYPE: sys::VariantType;

    /// # Safety
    /// - `variant` must be non-null.
    /// - `variant` must be a valid type of `Self`.
    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self;
}

// TODO: more types, operators
#[rmldoc(file = "api_Rml-Variant.md", name = "Rml::Variant")]
pub struct Variant<'a> {
    pub(crate) raw: Ptr<Self>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Raw for Variant<'a> {
    type Ptr = *const sys::Variant;

    fn raw(&self) -> Self::Ptr {
        self.raw
    }
}

#[rmldoc(file = "api_Rml-Variant.md")]
impl<'a> Variant<'a> {
    /// # Safety:
    /// - `raw` must be a valid non-null `Variant` pointer
    pub(crate) unsafe fn from_raw(raw: Ptr<Self>) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    #[rmldoc(name = "Rml::Variant::GetInto")]
    pub fn try_get_as<T: TypeVariant>(&self) -> Option<&'a T> {
        // Safety: `self.raw` is a valid `Variant`
        let ty = unsafe { sys::variant_get_type(self.raw()) };

        if ty != T::VARIANT_TYPE {
            return None;
        }

        // Safety: We have verified the variant is the correct type for `T`
        unsafe { Some(T::from_variant(self.raw())) }
    }

    pub fn get_as<T: TypeVariant>(&self) -> &'a T {
        self.try_get_as().expect("variant is not the expected type")
    }
}

#[sealed]
impl TypeVariant for bool {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::BOOL;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_bool(variant) }
    }
}

#[sealed]
impl TypeVariant for u8 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::BYTE;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_byte(variant) }
    }
}

#[sealed]
impl TypeVariant for f32 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::FLOAT;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_float(variant) }
    }
}

#[sealed]
impl TypeVariant for f64 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::DOUBLE;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_double(variant) }
    }
}

#[sealed]
impl TypeVariant for i32 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::INT;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_int(variant) }
    }
}

#[sealed]
impl TypeVariant for i64 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::INT64;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_int64(variant) }
    }
}

#[sealed]
impl TypeVariant for u32 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::UINT;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_uint(variant) }
    }
}

#[sealed]
impl TypeVariant for u64 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::UINT64;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_uint64(variant) }
    }
}

#[sealed]
impl TypeVariant for Vec2 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::VECTOR2;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { FromSys::from_sys(sys::variant_as_vector2f(variant)) }
    }
}

#[sealed]
impl TypeVariant for Vec3 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::VECTOR3;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { FromSys::from_sys(sys::variant_as_vector3f(variant)) }
    }
}

#[sealed]
impl TypeVariant for Vec4 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::VECTOR4;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { FromSys::from_sys(sys::variant_as_vector4f(variant)) }
    }
}
