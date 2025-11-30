use glam::{IVec2, Vec2};
use rsmlui_sys::Rml_Vector2f;

impl From<IVec2> for Rml_Vector2i {
    fn from(value: IVec2) -> Self {
        Rml_Vector2i {
            _phantom_0: std::marker::PhantomData,
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vec2> for Rml_Vector2f {
    fn from(value: Vec2) -> Self {
        Rml_Vector2f {
            _phantom_0: std::marker::PhantomData,
            x: value.x,
            y: value.y,
        }
    }
}
