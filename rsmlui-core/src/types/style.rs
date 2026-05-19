use rsmlui_macros::sys_cast;
use rsmlui_sys::{Rml_Style_FontStyle, Rml_Style_FontWeight};

#[non_exhaustive]
#[sys_cast(enum(from = Rml_Style_FontStyle, repr = u8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal = 0,
    Italic = 1,
}

#[non_exhaustive]
#[sys_cast(enum(from = Rml_Style_FontWeight, repr = u16))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Auto = 0,
    Normal = 400,
    Bold = 700,
}
