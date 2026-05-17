use glam::Vec2;
use rsmlui_macros::rmldoc;
use rsmlui_sys::{Rml_ColorStop, Rml_NumericValue, Rml_Unit, Rml_Vertex};

use crate::types::colour::ColorbPremultiplied;
use crate::utils::conversions::{FromSys, IntoSys};

pub type Unit = Rml_Unit;

#[rmldoc(file = "api_Rml-Vertex.md", name = "Rml::Vertex")]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vertex {
    #[rmldoc(name = "Rml::Vertex::position")]
    pub position: Vec2,
    #[rmldoc(name = "Rml::Vertex::colour")]
    pub colour: ColorbPremultiplied,
    #[rmldoc(name = "Rml::Vertex::tex_coord")]
    pub tex_coord: Vec2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NumericValue {
    pub number: f32,
    pub unit: Unit,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorStop {
    pub(crate) color: ColorbPremultiplied,
    pub(crate) position: NumericValue,
}

pub type ColorStopList = Vec<ColorStop>;
pub type ColorStops = [ColorStop];

const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    rsmlui_sys::const_assert_eq!(size_of::<NumericValue>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<NumericValue>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(NumericValue, number), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(NumericValue, unit), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_NumericValue>(), 8);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_NumericValue>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_NumericValue, number), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_NumericValue, unit), 4);

    rsmlui_sys::const_assert_eq!(size_of::<ColorStop>(), 12);
    rsmlui_sys::const_assert_eq!(align_of::<ColorStop>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorStop, color), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorStop, position), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_ColorStop>(), 12);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_ColorStop>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_ColorStop, color), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_ColorStop, position), 4);

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

impl FromSys<Rml_NumericValue> for NumericValue {
    fn from_sys(value: Rml_NumericValue) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_NumericValue> for &NumericValue {
    fn from_sys(value: &Rml_NumericValue) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_NumericValue> for NumericValue {
    fn into_sys(self) -> Rml_NumericValue {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a Rml_NumericValue> for &'a NumericValue {
    fn into_sys(self) -> &'a Rml_NumericValue {
        unsafe { std::mem::transmute(self) }
    }
}

impl FromSys<Rml_ColorStop> for ColorStop {
    fn from_sys(value: Rml_ColorStop) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_ColorStop> for &ColorStop {
    fn from_sys(value: &Rml_ColorStop) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&[Rml_ColorStop]> for &[ColorStop] {
    fn from_sys(value: &[Rml_ColorStop]) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_ColorStop> for ColorStop {
    fn into_sys(self) -> Rml_ColorStop {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a Rml_ColorStop> for &'a ColorStop {
    fn into_sys(self) -> &'a Rml_ColorStop {
        unsafe { std::mem::transmute(self) }
    }
}

impl FromSys<Rml_Vertex> for Vertex {
    fn from_sys(value: Rml_Vertex) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&[Rml_Vertex]> for &[Vertex] {
    fn from_sys(value: &[Rml_Vertex]) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl IntoSys<Rml_Vertex> for Vertex {
    fn into_sys(self) -> Rml_Vertex {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> IntoSys<&'a [Rml_Vertex]> for &'a [Vertex] {
    fn into_sys(self) -> &'a [Rml_Vertex] {
        unsafe { std::mem::transmute(self) }
    }
}
