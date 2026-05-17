use cxx::{ExternType, type_id};

unsafe impl ExternType for crate::Rml_Vector2f {
    type Id = type_id!("Rml::Vector2f");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Vector2i {
    type Id = type_id!("Rml::Vector2i");
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

unsafe impl ExternType for crate::Rml_Vector4i {
    type Id = type_id!("Rml::Vector4i");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Matrix4f {
    type Id = type_id!("Rml::Matrix4f");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Rectanglei {
    type Id = type_id!("Rml::Rectanglei");
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

unsafe impl ExternType for crate::Rml_Vertex {
    type Id = type_id!("Rml::Vertex");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_BlendMode {
    type Id = type_id!("Rml::BlendMode");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_ClipMaskOperation {
    type Id = type_id!("Rml::ClipMaskOperation");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_ColorStop {
    type Id = type_id!("Rml::ColorStop");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Input_KeyIdentifier {
    type Id = type_id!("Rml::Input::KeyIdentifier");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Input_KeyModifier {
    type Id = type_id!("Rml::Input::KeyModifier");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_ModalFlag {
    type Id = type_id!("Rml::ModalFlag");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_FocusFlag {
    type Id = type_id!("Rml::FocusFlag");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_ScrollFlag {
    type Id = type_id!("Rml::ScrollFlag");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Style_FontWeight {
    type Id = type_id!("Rml::Style::FontWeight");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Style_FontStyle {
    type Id = type_id!("Rml::Style::FontStyle");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Variant_Type {
    type Id = type_id!("Rml::Variant::Type");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Log_Type {
    type Id = type_id!("rsmlui::log::RmlLogType");
    type Kind = cxx::kind::Trivial;
}
