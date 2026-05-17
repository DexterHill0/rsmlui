use std::marker::PhantomData;

impl crate::Rml_Vector2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            _phantom_0: PhantomData,
            x,
            y,
        }
    }
}

impl crate::Rml_Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            _phantom_0: PhantomData,
            x,
            y,
        }
    }
}
