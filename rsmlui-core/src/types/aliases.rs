use rsmlui_macros::rmldoc;
use rsmlui_sys::{
    Rml_BlendMode, Rml_ClipMaskOperation, Rml_FocusFlag, Rml_Input_KeyIdentifier,
    Rml_Input_KeyModifier, Rml_ModalFlag, Rml_ScrollFlag, Rml_Style_FontStyle,
    Rml_Style_FontWeight, Rml_Unit,
};

#[rmldoc(file = "api_Rml.md", name = "Rml::ModalFlag")]
pub type ModalFlag = Rml_ModalFlag;

#[rmldoc(file = "api_Rml.md", name = "Rml::FocusFlag")]
pub type FocusFlag = Rml_FocusFlag;

#[rmldoc(file = "api_Rml.md", name = "Rml::ScrollFlag")]
pub type ScrollFlag = Rml_ScrollFlag;

pub type KeyCode = Rml_Input_KeyIdentifier;
pub type KeyModifier = Rml_Input_KeyModifier;

pub type FontStyle = Rml_Style_FontStyle;
pub type FontWeight = Rml_Style_FontWeight;

pub type BlendMode = Rml_BlendMode;
pub type ClipMaskOperation = Rml_ClipMaskOperation;
pub type Unit = Rml_Unit;
