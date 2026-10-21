use crate::math::UVec2;

pub trait RenderDriver {
    fn set_viewport(&mut self, dimensions: UVec2);

    fn begin_frame(&mut self);
    fn end_frame(&mut self);
    fn clear(&mut self);
}
