use std::marker::PhantomData;

use cxx::{type_id, ExternType};

unsafe impl ExternType for crate::Rml_Vector2f {
    type Id = type_id!("Rml::Vector2f");
    type Kind = cxx::kind::Trivial;
}

impl crate::Rml_Vector2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            _phantom_0: PhantomData,
            x,
            y,
        }
    }
}

unsafe impl ExternType for crate::Rml_Vector2i {
    type Id = type_id!("Rml::Vector2i");
    type Kind = cxx::kind::Trivial;
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
