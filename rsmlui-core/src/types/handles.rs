use rsmlui_macros::sys_cast;
use rsmlui_sys::{
    Rml_CompiledFilterHandle, Rml_CompiledGeometryHandle, Rml_CompiledShaderHandle,
    Rml_LayerHandle, Rml_TextureHandle,
};

#[sys_cast(struct(transparent, from = Rml_CompiledShaderHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledGeometryHandle(pub(crate) Rml_CompiledGeometryHandle);

#[sys_cast(struct(transparent, from = Rml_CompiledShaderHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledShaderHandle(pub(crate) Rml_CompiledShaderHandle);

#[sys_cast(struct(transparent, from = Rml_CompiledFilterHandle), gen_slice)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledFilterHandle(pub(crate) Rml_CompiledFilterHandle);

#[sys_cast(struct(transparent, from = Rml_LayerHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerHandle(pub(crate) Rml_LayerHandle);

#[sys_cast(struct(transparent, from = Rml_TextureHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureHandle(pub(crate) Rml_TextureHandle);

impl TextureHandle {
    // As per docs:
    // > The value zero (0) is reserved for invalid handles, and should only be used to indicate an error while trying to load the texture.
    /// Represents an invalid texture handle. It should only be used to indicate an error while trying to load the texture.
    pub const INVALID: Self = Self(0);
}
