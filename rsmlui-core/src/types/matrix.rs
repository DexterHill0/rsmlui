use glam::Mat4;
use rsmlui_sys::Rml_Matrix4f;

use crate::FromSys;

const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    // Mat4 / Rml_Matrix4f
    // NOTE: glam has `scalar-math` enabled which means it doesn't use SIMD-representation
    // so it has the same 4-byte alignment as RmlUi, rather than 16-byte, allowing the casts
    // to be sound. Both types are also column-major (4 columns of 4 floats).
    rsmlui_sys::const_assert_eq!(size_of::<Mat4>(), 64);
    rsmlui_sys::const_assert_eq!(align_of::<Mat4>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, x_axis), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, y_axis), 16);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, z_axis), 32);
    rsmlui_sys::const_assert_eq!(offset_of!(Mat4, w_axis), 48);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Matrix4f>(), 64);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Matrix4f>(), 4);
};

impl FromSys<Mat4> for Rml_Matrix4f {
    fn from_sys(value: Mat4) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Mat4> for &Rml_Matrix4f {
    fn from_sys(value: &Mat4) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

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
