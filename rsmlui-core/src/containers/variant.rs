use rsmlui_macros::rmldoc;
use rsmlui_sys::variant::{self as sys};
use sealed::sealed;

use crate::_private::FromSys;
use crate::math::{Vec2, Vec3, Vec4};
use crate::object::constness::{Constness, Immut};
use crate::object::foreign::{ForeignHandle, ForeignObject, ObjectPtr};
use crate::object::ownership::{Foreign, ObjectOwnership};
use crate::types::colour::Colorb;
use crate::types::renderer::ColorStop;

#[sealed]
pub trait TypeVariant {
    const VARIANT_TYPE: sys::VariantType;

    /// # Safety
    /// - `variant` must be non-null.
    /// - `variant` must be a valid type of `Self`.
    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self;
}

pub mod tags {
    pub enum TagVariant {}
}

impl ForeignObject for tags::TagVariant {
    type Object = sys::Variant;
}

/// Shared handle newtype for both [`Variant`] and [`VariantRef`].
#[repr(transparent)]
pub struct VariantHandle<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness>(
    ForeignHandle<'a, tags::TagVariant, M, C>,
);

// // TODO: more types, operators
// #[rmldoc(file = "api_Rml-Variant.md", name = "Rml::Variant")]
// pub type Variant = VariantHandle<'static, Native, Mut>;

/// A borrowed version of [`Variant`].
// TODO: more types, operators
#[rmldoc(file = "api_Rml-Variant.md", name = "Rml::Variant")]
pub type VariantRef<'a> = VariantHandle<'a, Foreign, Immut>;

impl<'a> VariantRef<'a> {
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `Variant` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectPtr<tags::TagVariant>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, Immut>::from_raw(raw) })
    }
}

#[rmldoc(file = "api_Rml-Variant.md")]
impl<'a> VariantRef<'a> {
    #[rmldoc(name = "Rml::Variant::GetInto")]
    pub fn try_get_as<T: TypeVariant + ?Sized>(&self) -> Option<&'a T> {
        // Safety: `self.raw` is a valid `Variant`
        let ty = unsafe { sys::variant_get_type(self.0.as_ptr()) };

        if ty != T::VARIANT_TYPE {
            return None;
        }

        // Safety: We have verified the variant is the correct type for `T`
        unsafe { Some(T::from_variant(self.0.as_ptr())) }
    }

    /// # Panics
    ///
    /// Use [`VariantHandle::try_get_as`] for a non-panicking version.
    pub fn get_as<T: TypeVariant + ?Sized>(&self) -> &'a T {
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

#[sealed]
impl TypeVariant for Colorb {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::COLOURB;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { FromSys::from_sys(sys::variant_as_colorb(variant)) }
    }
}

#[sealed]
impl TypeVariant for [ColorStop] {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::COLORSTOPLIST;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { FromSys::from_sys(sys::variant_as_color_stop_list(variant)) }
    }
}

#[sealed]
impl TypeVariant for str {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::STRING;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_str(variant) }
    }
}
