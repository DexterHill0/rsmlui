use glam::{IVec2, IVec4, Mat4, Vec2, Vec3, Vec4};
use rsmlui_sys::{
    Rml_Matrix4f, Rml_Vector2f, Rml_Vector2i, Rml_Vector3f, Rml_Vector4f, Rml_Vector4i,
};

use crate::utils::conversions::{FromSys, IntoSys};

const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    // Vec2 / Rml_Vector2f
    rsmlui_sys::const_assert_eq!(size_of::<Vec2>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Vec2>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec2, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec2, y), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector2f>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector2f>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2f, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2f, y), 4);

    // IVec2 / Rml_Vector2i
    rsmlui_sys::const_assert_eq!(size_of::<IVec2>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<IVec2>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec2, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec2, y), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector2i>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector2i>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2i, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2i, y), 4);

    // Vec3 / Rml_Vector3f
    rsmlui_sys::const_assert_eq!(size_of::<Vec3>(), 12);
    rsmlui_sys::const_assert_eq!(align_of::<Vec3>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec3, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec3, y), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec3, z), 8);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector3f>(), 12);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector3f>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector3f, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector3f, y), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector3f, z), 8);

    // Vec4 / Rml_Vector4f
    // NOTE: glam has `scalar-math` enabled which means it doesn't use SIMD-representation
    // so it has the same 4-byte alignment as RmlUi, rather than 16-byte, allowing the casts
    // to be sound
    rsmlui_sys::const_assert_eq!(size_of::<Vec4>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Vec4>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec4, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec4, y), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec4, z), 8);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec4, w), 12);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector4f>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector4f>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4f, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4f, y), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4f, z), 8);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4f, w), 12);

    // IVec4 / Rml_Vector4i
    rsmlui_sys::const_assert_eq!(size_of::<IVec4>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<IVec4>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec4, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec4, y), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec4, z), 8);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec4, w), 12);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector4i>(), 16);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector4i>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4i, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4i, y), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4i, z), 8);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector4i, w), 12);

    // Mat4 / Rml_Matrix4f
    // Both types are column-major (4 columns of 4 floats).
    rsmlui_sys::const_assert_eq!(size_of::<Mat4>(), 64);
    rsmlui_sys::const_assert_eq!(align_of::<Mat4>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, x_axis), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, y_axis), 16);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, z_axis), 32);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, w_axis), 48);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Matrix4f>(), 64);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Matrix4f>(), 4);
};

// Vec2
impl FromSys<Rml_Vector2f> for Vec2 {
    fn from_sys(value: Rml_Vector2f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Vector2f> for &Vec2 {
    fn from_sys(value: &Rml_Vector2f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Vector2f> for Vec2 {
    fn into_sys(self) -> Rml_Vector2f {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a Rml_Vector2f> for &'a Vec2 {
    fn into_sys(self) -> &'a Rml_Vector2f {
        unsafe { std::mem::transmute(self) }
    }
}

// IVec2
impl FromSys<Rml_Vector2i> for IVec2 {
    fn from_sys(value: Rml_Vector2i) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Vector2i> for IVec2 {
    fn into_sys(self) -> Rml_Vector2i {
        unsafe { std::mem::transmute(self) }
    }
}

// Vec3
impl FromSys<Rml_Vector3f> for Vec3 {
    fn from_sys(value: Rml_Vector3f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Vector3f> for &Vec3 {
    fn from_sys(value: &Rml_Vector3f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Vector3f> for Vec3 {
    fn into_sys(self) -> Rml_Vector3f {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a Rml_Vector3f> for &'a Vec3 {
    fn into_sys(self) -> &'a Rml_Vector3f {
        unsafe { std::mem::transmute(self) }
    }
}

// Vec4
impl FromSys<Rml_Vector4f> for Vec4 {
    fn from_sys(value: Rml_Vector4f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Vector4f> for &Vec4 {
    fn from_sys(value: &Rml_Vector4f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Vector4f> for Vec4 {
    fn into_sys(self) -> Rml_Vector4f {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a Rml_Vector4f> for &'a Vec4 {
    fn into_sys(self) -> &'a Rml_Vector4f {
        unsafe { std::mem::transmute(self) }
    }
}

// IVec4
impl FromSys<Rml_Vector4i> for IVec4 {
    fn from_sys(value: Rml_Vector4i) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Vector4i> for IVec4 {
    fn into_sys(self) -> Rml_Vector4i {
        unsafe { std::mem::transmute(self) }
    }
}

// Mat4
impl FromSys<Rml_Matrix4f> for Mat4 {
    fn from_sys(value: Rml_Matrix4f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_Matrix4f> for &Mat4 {
    fn from_sys(value: &Rml_Matrix4f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Matrix4f> for Mat4 {
    fn into_sys(self) -> Rml_Matrix4f {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a Rml_Matrix4f> for &'a Mat4 {
    fn into_sys(self) -> &'a Rml_Matrix4f {
        unsafe { std::mem::transmute(self) }
    }
}
