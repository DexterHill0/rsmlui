use rsmlui_core::math::UVec2;

#[cfg(feature = "renderer-gl2")]
pub mod gl2;
#[cfg(feature = "renderer-gl3")]
pub mod gl3;

pub trait RenderDriver {
    fn set_viewport(&mut self, dimensions: impl Into<UVec2>);
    fn clear(&mut self);
    fn begin_frame(&mut self);
    fn end_frame(&mut self);
}
