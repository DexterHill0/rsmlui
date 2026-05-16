use glam::Vec2;
use rsmlui_macros::rmldoc;
use rsmlui_sys::{
    Rml_BlendMode, Rml_ClipMaskOperation, Rml_ColorStop, Rml_CompiledFilterHandle,
    Rml_CompiledGeometryHandle, Rml_CompiledShaderHandle, Rml_LayerHandle, Rml_NumericValue,
    Rml_TextureHandle, Rml_Unit, Rml_Vertex,
};

use crate::types::colour::ColorbPremultiplied;
use crate::utils::conversions::FromSys;

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

pub type CompiledGeometryHandle = Rml_CompiledGeometryHandle;
pub type CompiledShaderHandle = Rml_CompiledShaderHandle;
pub type CompiledFilterHandle = Rml_CompiledFilterHandle;
pub type LayerHandle = Rml_LayerHandle;
pub type TextureHandle = Rml_TextureHandle;
pub type BlendMode = Rml_BlendMode;
pub type ClipMaskOperation = Rml_ClipMaskOperation;

pub type Unit = Rml_Unit;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct NumericValue {
    pub number: f32,
    pub unit: Unit,
}

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
};

impl FromSys<Rml_NumericValue> for NumericValue {
    fn from_sys(value: Rml_NumericValue) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_NumericValue> for &NumericValue {
    fn from_sys(value: &Rml_NumericValue) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<NumericValue> for Rml_NumericValue {
    fn from_sys(value: NumericValue) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&NumericValue> for &Rml_NumericValue {
    fn from_sys(value: &NumericValue) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorStop {
    color: ColorbPremultiplied,
    position: NumericValue,
}

pub type ColorStopList = Vec<ColorStop>;
pub type ColorStops = [ColorStop];

const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    rsmlui_sys::const_assert_eq!(size_of::<ColorStop>(), 12);
    rsmlui_sys::const_assert_eq!(align_of::<ColorStop>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorStop, color), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(ColorStop, position), 4);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_ColorStop>(), 12);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_ColorStop>(), 4);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_ColorStop, color), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Rml_ColorStop, position), 4);
};

impl FromSys<Rml_ColorStop> for ColorStop {
    fn from_sys(value: Rml_ColorStop) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&Rml_ColorStop> for &ColorStop {
    fn from_sys(value: &Rml_ColorStop) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&[Rml_ColorStop]> for &[ColorStop] {
    fn from_sys(value: &[Rml_ColorStop]) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<ColorStop> for Rml_ColorStop {
    fn from_sys(value: ColorStop) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<&ColorStop> for &Rml_ColorStop {
    fn from_sys(value: &ColorStop) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

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
