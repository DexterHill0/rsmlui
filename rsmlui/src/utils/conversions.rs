use glam::{IVec2, Vec2};
use rsmlui_sys::{Rml_Vector2f, Rml_Vector2i};

pub(crate) trait IntoSys {
    type Output;

    fn into_sys(self) -> Self::Output;
}

impl IntoSys for IVec2 {
    type Output = Rml_Vector2i;

    fn into_sys(self) -> Self::Output {
        Rml_Vector2i {
            _phantom_0: std::marker::PhantomData,
            x: self.x,
            y: self.y,
        }
    }
}

impl IntoSys for Vec2 {
    type Output = Rml_Vector2f;

    fn into_sys(self) -> Self::Output {
        Rml_Vector2f {
            _phantom_0: std::marker::PhantomData,
            x: self.x,
            y: self.y,
        }
    }
}
