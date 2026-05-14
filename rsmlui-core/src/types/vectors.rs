use glam::{IVec2, Vec2};
use rsmlui_sys::{Rml_Vector2f, Rml_Vector2i};

use crate::FromSys;

const _: () = {
    use std::mem::{align_of, offset_of, size_of};
    rsmlui_sys::const_assert_eq!(size_of::<Vec2>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Vec2>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec2, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Vec2, y), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector2f>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector2f>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2f, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2f, y), 4);
    rsmlui_sys::const_assert_eq!(size_of::<IVec2>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<IVec2>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec2, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(IVec2, y), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vector2i>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vector2i>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2i, x), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vector2i, y), 4);
};

impl FromSys<IVec2> for Rml_Vector2i {
    fn from_sys(value: IVec2) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Vec2> for Rml_Vector2f {
    fn from_sys(value: Vec2) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rml_Vector2i> for IVec2 {
    fn from_sys(value: Rml_Vector2i) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rml_Vector2f> for Vec2 {
    fn from_sys(value: Rml_Vector2f) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
