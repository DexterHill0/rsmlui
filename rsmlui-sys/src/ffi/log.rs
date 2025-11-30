use cxx::{ExternType, type_id};

unsafe impl ExternType for crate::Rml_Log_Type {
    type Id = type_id!("Rml::Log::Type");
    type Kind = cxx::kind::Trivial;
}
