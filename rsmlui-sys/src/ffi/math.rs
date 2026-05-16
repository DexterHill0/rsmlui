use std::marker::PhantomData;

use cxx::{ExternType, type_id};

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

unsafe impl ExternType for crate::Rml_Rectanglei {
    type Id = type_id!("Rml::Rectanglei");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Vector3f {
    type Id = type_id!("Rml::Vector3f");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Vector4f {
    type Id = type_id!("Rml::Vector4f");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Matrix4f {
    type Id = type_id!("Rml::Matrix4f");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Colourb {
    type Id = type_id!("Rml::Colourb");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_ColourbPremultiplied {
    type Id = type_id!("Rml::ColourbPremultiplied");
    type Kind = cxx::kind::Trivial;
}
