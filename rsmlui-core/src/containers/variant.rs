use std::fmt::Debug;

use rsmlui_macros::rmldoc;
use rsmlui_sys::variant::{self as sys};
use sealed::sealed;

use crate::_private::{FromSys, IntoSys};
use crate::math::{Vec2, Vec3, Vec4};
use crate::object::constness::{Const, Constness, NonConst};
use crate::object::foreign::{
    ForeignDestroy, ForeignHandle, ForeignObject, ObjectMutPtr, ObjectPtr,
};
use crate::object::ownership::{Foreign, Native, ObjectOwnership};
use crate::types::colour::Colorb;
use crate::types::renderer::ColorStop;

#[sealed]
pub trait TypeVariant: ToOwned {
    const VARIANT_TYPE: sys::VariantType;

    /// # Safety
    /// - `variant` must be non-null.
    /// - `variant` must be a valid type of `Self`.
    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned;
}

#[sealed]
pub trait TypeVariantSet<T: ?Sized> {
    fn set(&mut self, value: T);
}

pub mod tags {
    #[derive(Debug)]
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
pub type Variant = VariantHandle<'static, Native, NonConst>;

/// An immutably borrowed version of [`Variant`].
pub type VariantRef<'a> = VariantHandle<'a, Foreign, Const>;

/// An mutably borrowed version of [`Variant`].
pub type VariantRefMut<'a> = VariantHandle<'a, Foreign, NonConst>;

impl Default for Variant {
    fn default() -> Self {
        // Safety: the allocation is a brand new object and not a reference to
        // any existing data.
        Self(unsafe { ForeignHandle::from_raw(sys::new_variant()) })
    }
}

// Only native variants can be created
#[rmldoc(file = "api_Rml-Variant.md")]
impl Variant {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness> VariantHandle<'a, M, C> {
    /// Returns the raw C++ variant pointer.
    ///
    /// # Safety
    ///
    /// The pointer is valid for the lifetime of this `Variant`. The caller must not
    /// store it beyond that lifetime, and must not use it after `Variant` is dropped.
    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> ObjectPtr<tags::TagVariant> {
        unsafe { self.0.as_ptr() }
    }
}

impl<'a, M: ObjectOwnership<tags::TagVariant>> VariantHandle<'a, M, NonConst> {
    /// Returns the raw C++ variant pointer.
    ///
    /// # Safety
    ///
    /// The pointer is valid for the lifetime of this `Variant`. The caller must not
    /// store it beyond that lifetime, and must not use it after `Variant` is dropped.
    #[doc(hidden)]
    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> ObjectMutPtr<tags::TagVariant> {
        unsafe { self.0.as_mut_ptr() }
    }
}

// As per `ForeignHandle` docs, we can only return objects with `'a` lifetime
// when `M = Foreign` and `C = Const`, as that allows the object to outlive
// the handle ref
#[rmldoc(file = "api_Rml-Variant.md")]
impl<'a> VariantRef<'a> {
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `Variant` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectPtr<tags::TagVariant>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, Const>::from_raw(raw) })
    }
}

#[rmldoc(file = "api_Rml-Variant.md")]
impl<'a> VariantRefMut<'a> {
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `Variant` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectMutPtr<tags::TagVariant>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, NonConst>::from_raw(raw) })
    }
}

#[rmldoc(file = "api_Rml-Variant.md")]
impl<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness> VariantHandle<'a, M, C> {
    #[rmldoc(name = "Rml::Variant::GetInto")]
    pub fn try_get_as<T: TypeVariant + ?Sized>(&self) -> Option<<T as ToOwned>::Owned> {
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
    pub fn get_as<T: TypeVariant + ?Sized>(&self) -> <T as ToOwned>::Owned {
        self.try_get_as::<T>()
            .expect("variant is not the expected type")
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

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_bool(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for u8 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::BYTE;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_byte(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for i8 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::CHAR;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_char(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for f32 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::FLOAT;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_float(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for f64 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::DOUBLE;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_double(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for i32 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::INT;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_int(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for i64 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::INT64;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_int64(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for u32 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::UINT;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_uint(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for u64 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::UINT64;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_uint64(variant).to_owned() }
    }
}

#[sealed]
impl TypeVariant for Vec2 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::VECTOR2;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        let vec: &Vec2 = unsafe { FromSys::from_sys(sys::variant_as_vector2f(variant)) };

        vec.to_owned()
    }
}

#[sealed]
impl TypeVariant for Vec3 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::VECTOR3;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        let vec: &Vec3 = unsafe { FromSys::from_sys(sys::variant_as_vector3f(variant)) };

        vec.to_owned()
    }
}

#[sealed]
impl TypeVariant for Vec4 {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::VECTOR4;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        let vec: &Vec4 = unsafe { FromSys::from_sys(sys::variant_as_vector4f(variant)) };

        vec.to_owned()
    }
}

#[sealed]
impl TypeVariant for Colorb {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::COLOURB;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        let colorb: &Colorb = unsafe { FromSys::from_sys(sys::variant_as_colorb(variant)) };

        colorb.to_owned()
    }
}

#[sealed]
impl TypeVariant for [ColorStop] {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::COLORSTOPLIST;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        let stops: &[ColorStop] =
            unsafe { FromSys::from_sys(sys::variant_as_color_stop_list(variant)) };

        stops.to_owned()
    }
}

#[sealed]
impl TypeVariant for str {
    const VARIANT_TYPE: sys::VariantType = sys::VariantType::STRING;

    unsafe fn from_variant(variant: *const sys::Variant) -> <Self as ToOwned>::Owned {
        unsafe { sys::variant_as_str(variant).to_owned() }
    }
}

// setters

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<bool>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: bool) {
        unsafe {
            sys::variant_set_bool(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<u8>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: u8) {
        unsafe {
            sys::variant_set_byte(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<i8>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: i8) {
        unsafe {
            sys::variant_set_char(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<f32>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: f32) {
        unsafe {
            sys::variant_set_float(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<f64>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: f64) {
        unsafe {
            sys::variant_set_double(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<i32>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: i32) {
        unsafe {
            sys::variant_set_int(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<i64>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: i64) {
        unsafe {
            sys::variant_set_int64(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<u32>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: u32) {
        unsafe {
            sys::variant_set_uint(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<u64>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: u64) {
        unsafe {
            sys::variant_set_uint64(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<Vec2>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: Vec2) {
        unsafe {
            sys::variant_set_vector2f(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<Vec3>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: Vec3) {
        unsafe {
            sys::variant_set_vector3f(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<Vec4>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: Vec4) {
        unsafe {
            sys::variant_set_vector4f(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<Colorb>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: Colorb) {
        unsafe {
            sys::variant_set_colorb(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<&[ColorStop]>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: &[ColorStop]) {
        unsafe {
            sys::variant_set_color_stop_list(self.0.as_mut_ptr(), value.into_sys());
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<&str>
    for VariantHandle<'a, M, NonConst>
{
    fn set(&mut self, value: &str) {
        unsafe {
            sys::variant_set_str(self.0.as_mut_ptr(), value);
        }
    }
}

#[sealed]
impl<'a, M: ObjectOwnership<tags::TagVariant>> TypeVariantSet<Variant>
    for VariantHandle<'a, M, NonConst>
{
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
impl<'a, M: ObjectOwnership<tags::TagVariant>, T> TypeVariantSet<&T>
    for VariantHandle<'a, M, NonConst>
where
    Self: TypeVariantSet<T>,
    T: Clone,
{
    fn set(&mut self, value: &T) {
        <VariantHandle<'a, M, NonConst> as TypeVariantSet<T>>::set(self, value.clone())
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;

    impl<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness> VariantHandle<'a, M, C> {
        fn ty(&self) -> sys::VariantType {
            unsafe { sys::variant_get_type(self.0.as_ptr()) }
        }
    }

    fn variant_type_to_str(ty: sys::VariantType) -> &'static str {
        match ty {
            sys::VariantType::NONE => "<unset>",
            sys::VariantType::BOOL => "bool",
            sys::VariantType::BYTE => "u8",
            sys::VariantType::CHAR => "char",
            sys::VariantType::FLOAT => "f32",
            sys::VariantType::DOUBLE => "f64",
            sys::VariantType::INT => "i32",
            sys::VariantType::INT64 => "i64",
            sys::VariantType::UINT => "u32",
            sys::VariantType::UINT64 => "u64",
            sys::VariantType::STRING => "str",
            sys::VariantType::VECTOR2 => "Vec2",
            sys::VariantType::VECTOR3 => "Vec3",
            sys::VariantType::VECTOR4 => "Vec4",
            sys::VariantType::COLOURB => "Colourb",
            sys::VariantType::COLORSTOPLIST => "Vec<ColorStop>",
            _ => unimplemented!(),
        }
    }

    fn variant_value_to_string(ty: sys::VariantType, variant: *const sys::Variant) -> String {
        unsafe {
            match ty {
                sys::VariantType::NONE => "None".to_owned(),
                sys::VariantType::BOOL => {
                    format!("{:?}", <bool as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::CHAR => {
                    format!("{:?}", <i8 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::BYTE => {
                    format!("{:?}", <u8 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::FLOAT => {
                    format!("{:?}", <f32 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::DOUBLE => {
                    format!("{:?}", <f64 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::INT => {
                    format!("{:?}", <i32 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::INT64 => {
                    format!("{:?}", <i64 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::UINT => {
                    format!("{:?}", <u32 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::UINT64 => {
                    format!("{:?}", <u64 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::STRING => <str as TypeVariant>::from_variant(variant),
                sys::VariantType::VECTOR2 => {
                    format!("{:?}", <Vec2 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::VECTOR3 => {
                    format!("{:?}", <Vec3 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::VECTOR4 => {
                    format!("{:?}", <Vec4 as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::COLOURB => {
                    format!("{:?}", <Colorb as TypeVariant>::from_variant(variant))
                },
                sys::VariantType::COLORSTOPLIST => {
                    format!("{:?}", <[ColorStop] as TypeVariant>::from_variant(variant))
                },
                _ => unimplemented!(),
            }
        }
    }

    #[cfg(feature = "printing")]
    impl<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness> Debug for VariantHandle<'a, M, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let ty = self.ty();

            f.debug_struct("Variant")
                .field("type", &variant_type_to_str(ty))
                .field(
                    "value",
                    &variant_value_to_string(ty, unsafe { self.0.as_ptr() }),
                )
                .finish()
        }
    }
}

#[cfg(not(feature = "printing"))]
impl<'a, M: ObjectOwnership<tags::TagVariant>, C: Constness> Debug for VariantHandle<'a, M, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variant {{ <foreign> }}")
    }
}
