use std::marker::PhantomData;

use glam::{IVec2, Vec2};
use rsmlui_sys::{Rml_Vector2f, Rml_Vector2i};

pub trait IntoSys<T> {
    fn into_sys(self) -> T;
}

pub trait FromSys<T>: Sized {
    fn from_sys(value: T) -> Self;
}

impl<T, U> IntoSys<U> for T
where
    U: FromSys<T>,
{
    fn into_sys(self) -> U {
        U::from_sys(self)
    }
}

impl FromSys<IVec2> for Rml_Vector2i {
    fn from_sys(value: IVec2) -> Self {
        Rml_Vector2i {
            _phantom_0: PhantomData,
            x: value.x,
            y: value.y,
        }
    }
}

impl FromSys<Vec2> for Rml_Vector2f {
    fn from_sys(value: Vec2) -> Self {
        Rml_Vector2f {
            _phantom_0: PhantomData,
            x: value.x,
            y: value.y,
        }
    }
}

impl FromSys<Rml_Vector2i> for IVec2 {
    fn from_sys(value: Rml_Vector2i) -> Self {
        IVec2::new(value.x, value.y)
    }
}

impl FromSys<Rml_Vector2f> for Vec2 {
    fn from_sys(value: Rml_Vector2f) -> Self {
        Vec2::new(value.x, value.y)
    }
}
