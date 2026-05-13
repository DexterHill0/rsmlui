use rsmlui_macros::rmldoc;
use rsmlui_sys::{Rml_FocusFlag, Rml_ModalFlag, Rml_ScrollFlag};

#[rmldoc(file = "api_Rml.md", name = "Rml::ModalFlag")]
pub type ModalFlag = Rml_ModalFlag;

#[rmldoc(file = "api_Rml.md", name = "Rml::FocusFlag")]
pub type FocusFlag = Rml_FocusFlag;

#[rmldoc(file = "api_Rml.md", name = "Rml::ScrollFlag")]
pub type ScrollFlag = Rml_ScrollFlag;
