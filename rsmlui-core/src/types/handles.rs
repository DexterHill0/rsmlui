use std::num::NonZeroUsize;

use rsmlui_macros::sys_cast;
use rsmlui_sys::{
    Rml_CompiledFilterHandle, Rml_CompiledGeometryHandle, Rml_CompiledShaderHandle, Rml_FileHandle,
    Rml_LayerHandle, Rml_TextureHandle,
};

#[sys_cast(struct(transparent, from = Rml_CompiledShaderHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledGeometryHandle(pub(crate) Rml_CompiledGeometryHandle);

impl CompiledGeometryHandle {
    pub const fn new(id: NonZeroUsize) -> Self {
        Self(id.get())
    }
}

impl From<CompiledGeometryHandle> for usize {
    fn from(value: CompiledGeometryHandle) -> Self {
        value.0
    }
}

impl From<NonZeroUsize> for CompiledGeometryHandle {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}

#[sys_cast(struct(transparent, from = Rml_CompiledShaderHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledShaderHandle(pub(crate) Rml_CompiledShaderHandle);

impl CompiledShaderHandle {
    pub const fn new(id: NonZeroUsize) -> Self {
        Self(id.get())
    }
}

impl From<CompiledShaderHandle> for usize {
    fn from(value: CompiledShaderHandle) -> Self {
        value.0
    }
}

impl From<NonZeroUsize> for CompiledShaderHandle {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}

#[sys_cast(struct(transparent, from = Rml_CompiledFilterHandle), gen_slice)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompiledFilterHandle(pub(crate) Rml_CompiledFilterHandle);

impl CompiledFilterHandle {
    pub const fn new(id: NonZeroUsize) -> Self {
        Self(id.get())
    }
}

impl From<CompiledFilterHandle> for usize {
    fn from(value: CompiledFilterHandle) -> Self {
        value.0
    }
}

impl From<NonZeroUsize> for CompiledFilterHandle {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}

#[sys_cast(struct(transparent, from = Rml_LayerHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerHandle(pub(crate) Rml_LayerHandle);

impl LayerHandle {
    pub const fn new(id: NonZeroUsize) -> Self {
        Self(id.get())
    }
}

impl From<LayerHandle> for usize {
    fn from(value: LayerHandle) -> Self {
        value.0
    }
}

impl From<NonZeroUsize> for LayerHandle {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}

#[sys_cast(struct(transparent, from = Rml_TextureHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureHandle(pub(crate) Rml_TextureHandle);

impl TextureHandle {
    // As per docs:
    // > The value zero (0) is reserved for invalid handles, and should only be used to indicate an error while trying to load the texture.
    /// Represents an invalid texture handle. It should only be used to indicate an error while trying to load the texture.
    pub const INVALID: Self = Self(0);

    pub const fn new(id: NonZeroUsize) -> Self {
        Self(id.get())
    }
}

impl From<TextureHandle> for usize {
    fn from(value: TextureHandle) -> Self {
        value.0
    }
}

impl From<NonZeroUsize> for TextureHandle {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}

#[sys_cast(struct(transparent, from = Rml_FileHandle))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileHandle(pub(crate) Rml_FileHandle);

impl FileHandle {
    pub const INVALID: Self = Self(0);

    pub const fn new(id: NonZeroUsize) -> Self {
        Self(id.get())
    }
}

impl From<FileHandle> for usize {
    fn from(value: FileHandle) -> Self {
        value.0
    }
}

impl From<NonZeroUsize> for FileHandle {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}
