use rsmlui_macros::rmldoc;
use rsmlui_sys::variant::{self as sys};
use sealed::sealed;

use crate::_private::{FromSys, IntoSys};
use crate::math::{Vec2, Vec3, Vec4};
use crate::object::constness::{Constness, Immut, Mut};
use crate::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject, ObjectPtr};
use crate::object::ownership::{Foreign, Native, ObjectOwnership};
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

#[sealed]
pub trait TypeVariantSet<T: ?Sized> {
    fn set(&mut self, value: T);
}

pub mod tags {
    pub enum TagVariant {}
}

impl ForeignObject for tags::TagVariant {
    type Object = sys::Variant;
}

unsafe impl ForeignDestroy for tags::TagVariant {
    unsafe fn destroy(raw: *mut Self::Object) {
        // Safety: raw is a valid, non-null pointer
        unsafe { sys::variant_destructor(raw) }
    }
}

/// Shared handle newtype for both [`Variant`] and [`VariantRef`].
#[repr(transparent)]
pub struct VariantHandle<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness>(
    ForeignHandle<'a, tags::TagVariant, M, C>,
);

// This does not break the Aliasing rule of ForeignHandle as this always represents
// a newly constructed object, and never one borrowed from another object.
#[rmldoc(file = "api_Rml-Variant.md", name = "Rml::Variant")]
pub type Variant = VariantHandle<'static, Native, Mut>;

impl Default for Variant {
    fn default() -> Self {
        // Safety: the allocation is a brand new object and not a reference to
        // any existing data.
        Self(unsafe { ForeignHandle::from_raw(sys::new_variant()) })
    }
}

#[rmldoc(file = "api_Rml-Variant.md")]
impl Variant {
    pub fn new() -> Self {
        Self::default()
    }

    #[rmldoc(name = "Rml::Variant::GetInto")]
    pub fn try_get_as<T: TypeVariant + ?Sized>(&self) -> Option<&'_ T> {
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
    pub fn get_as<T: TypeVariant + ?Sized>(&self) -> &'_ T {
        self.try_get_as().expect("variant is not the expected type")
    }

    pub fn is<T: TypeVariant + ?Sized>(&self) -> bool {
        // Safety: `self.0.as_ptr()` is a valid `Variant`
        let ty = unsafe { sys::variant_get_type(self.0.as_ptr()) };

        ty == T::VARIANT_TYPE
    }
}

// impl<T> From<T> for Variant
// where
//     Variant: TypeVariantSet<T>,
// {
//     fn from(value: T) -> Self {
//         let mut variant = Self::new();
//         variant.set(value);
//         variant
//     }
// }

/// An immutably borrowed version of [`Variant`].
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

    pub fn is<T: TypeVariant + ?Sized>(&self) -> bool {
        // Safety: `self.0.as_ptr()` is a valid `Variant`
        let ty = unsafe { sys::variant_get_type(self.0.as_ptr()) };

        ty == T::VARIANT_TYPE
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
impl TypeVariant for i8 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::CHAR;

    unsafe fn from_variant<'a>(variant: *const sys::Variant) -> &'a Self {
        unsafe { sys::variant_as_char(variant) }
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

// setters

#[sealed]
impl TypeVariantSet<bool> for Variant {
    fn set(&mut self, value: bool) {
        unsafe {
            sys::variant_set_bool(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<u8> for Variant {
    fn set(&mut self, value: u8) {
        unsafe {
            sys::variant_set_byte(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<i8> for Variant {
    fn set(&mut self, value: i8) {
        unsafe {
            sys::variant_set_char(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<f32> for Variant {
    fn set(&mut self, value: f32) {
        unsafe {
            sys::variant_set_float(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<f64> for Variant {
    fn set(&mut self, value: f64) {
        unsafe {
            sys::variant_set_double(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<i32> for Variant {
    fn set(&mut self, value: i32) {
        unsafe {
            sys::variant_set_int(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<i64> for Variant {
    fn set(&mut self, value: i64) {
        unsafe {
            sys::variant_set_int64(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<u32> for Variant {
    fn set(&mut self, value: u32) {
        unsafe {
            sys::variant_set_uint(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<u64> for Variant {
    fn set(&mut self, value: u64) {
        unsafe {
            sys::variant_set_uint64(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<Vec2> for Variant {
    fn set(&mut self, value: Vec2) {
        unsafe {
            sys::variant_set_vector2f(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl TypeVariantSet<Vec3> for Variant {
    fn set(&mut self, value: Vec3) {
        unsafe {
            sys::variant_set_vector3f(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl TypeVariantSet<Vec4> for Variant {
    fn set(&mut self, value: Vec4) {
        unsafe {
            sys::variant_set_vector4f(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl TypeVariantSet<Colorb> for Variant {
    fn set(&mut self, value: Colorb) {
        unsafe {
            sys::variant_set_colorb(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl TypeVariantSet<&[ColorStop]> for Variant {
    fn set(&mut self, value: &[ColorStop]) {
        unsafe {
            sys::variant_set_color_stop_list(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl TypeVariantSet<&str> for Variant {
    fn set(&mut self, value: &str) {
        unsafe {
            sys::variant_set_str(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl TypeVariantSet<Variant> for Variant {
    fn set(&mut self, value: Variant) {
        let value_ptr = unsafe { value.0.as_ptr() };

        // Safety: RmlUi will copy the data from `value` into the `self`
        // so after this function is called, `value` has been copies and
        // the Variant is safe to drop
        unsafe {
            sys::variant_set_variant(self.0.as_mut_ptr(), value_ptr);
        }
    }
}

#[sealed]
impl<T> TypeVariantSet<&T> for Variant
where
    Variant: TypeVariantSet<T>,
    T: Clone,
{
    fn set(&mut self, value: &T) {
        <Variant as TypeVariantSet<T>>::set(self, value.clone())
    }
}
