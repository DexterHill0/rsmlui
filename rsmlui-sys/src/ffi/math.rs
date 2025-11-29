use cxx::{type_id, ExternType};

unsafe impl ExternType for crate::Rml_Vector2f {
    type Id = type_id!("Rml::Vector2f");
    type Kind = cxx::kind::Trivial;
}
