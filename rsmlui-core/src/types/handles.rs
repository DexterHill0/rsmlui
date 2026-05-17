use rsmlui_sys::{
    Rml_CompiledFilterHandle, Rml_CompiledGeometryHandle, Rml_CompiledShaderHandle,
    Rml_LayerHandle, Rml_TextureHandle,
};

use crate::{FromSys, IntoSys};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledGeometryHandle(pub(crate) Rml_CompiledGeometryHandle);

impl IntoSys<Rml_CompiledGeometryHandle> for CompiledGeometryHandle {
    fn into_sys(self) -> Rml_CompiledGeometryHandle {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledShaderHandle(pub(crate) Rml_CompiledShaderHandle);

impl IntoSys<Rml_CompiledShaderHandle> for CompiledShaderHandle {
    fn into_sys(self) -> Rml_CompiledShaderHandle {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledFilterHandle(pub(crate) Rml_CompiledFilterHandle);

impl IntoSys<Rml_CompiledFilterHandle> for CompiledFilterHandle {
    fn into_sys(self) -> Rml_CompiledFilterHandle {
        self.0
    }
}

impl<'a> IntoSys<&'a [Rml_CompiledFilterHandle]> for &'a [CompiledFilterHandle] {
    fn into_sys(self) -> &'a [Rml_CompiledFilterHandle] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> FromSys<&'a [Rml_CompiledFilterHandle]> for &'a [CompiledFilterHandle] {
    fn from_sys(value: &'a [Rml_CompiledFilterHandle]) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerHandle(pub(crate) Rml_LayerHandle);

impl IntoSys<Rml_LayerHandle> for LayerHandle {
    fn into_sys(self) -> Rml_LayerHandle {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureHandle(pub(crate) Rml_TextureHandle);

impl TextureHandle {
    // As per docs:
    // > The value zero (0) is reserved for invalid handles, and should only be used to indicate an error while trying to load the texture.
    /// Represents an invalid texture handle. It should only be used to indicate an error while trying to load the texture.
    pub const INVALID: Self = Self(0);
}

impl IntoSys<Rml_TextureHandle> for TextureHandle {
    fn into_sys(self) -> Rml_TextureHandle {
        self.0
    }
}
