use cxx::{type_id, ExternType};

unsafe impl ExternType for crate::Rml_Log_Type {
    type Id = type_id!("Rml::Log::Type");
    type Kind = cxx::kind::Trivial;
}
