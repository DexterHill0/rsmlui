use glam::Vec2;
use rsmlui_macros::rmldoc;
use rsmlui_sys::{Rml_CompiledGeometryHandle, Rml_Vertex};

use crate::types::colour::ColourbPremultiplied;
use crate::utils::conversions::FromSys;

#[rmldoc(file = "api_Rml-Vertex.md", name = "Rml::Vertex")]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vertex {
    #[rmldoc(name = "Rml::Vertex::position")]
    pub position: Vec2,
    #[rmldoc(name = "Rml::Vertex::colour")]
    pub colour: ColourbPremultiplied,
    #[rmldoc(name = "Rml::Vertex::tex_coord")]
    pub tex_coord: Vec2,
}

pub type CompiledGeometryHandle = Rml_CompiledGeometryHandle;

const _: () = {
    use std::mem::{align_of, offset_of, size_of};
    rsmlui_sys::const_assert_eq!(size_of::<Vertex>(), 20);
    rsmlui_sys::const_assert_eq!(align_of::<Vertex>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Vertex, position), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Vertex, colour), 8);
    rsmlui_sys::const_assert_eq!(offset_of!(Vertex, tex_coord), 12);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Vertex>(), 20);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Vertex>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vertex, position), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vertex, colour), 8);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_Vertex, tex_coord), 12);
};

impl FromSys<Vertex> for Rml_Vertex {
    fn from_sys(value: Vertex) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rml_Vertex> for Vertex {
    fn from_sys(value: Rml_Vertex) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a> FromSys<&'a [Vertex]> for &'a [Rml_Vertex] {
    fn from_sys(value: &'a [Vertex]) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a> FromSys<&'a [Rml_Vertex]> for &'a [Vertex] {
    fn from_sys(value: &'a [Rml_Vertex]) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}
