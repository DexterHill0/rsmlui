use rsmlui_macros::sys_cast;
use rsmlui_sys::{Rml_FocusFlag, Rml_ModalFlag, Rml_ScrollFlag};

#[non_exhaustive]
#[sys_cast(enum(from = Rml_ModalFlag, repr = i32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ModalFlag {
    None = 0,
    Modal = 1,
    Keep = 2,
}

#[non_exhaustive]
#[sys_cast(enum(from = Rml_FocusFlag, repr = i32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FocusFlag {
    None = 0,
    Document = 1,
    Keep = 2,
    Auto = 3,
}

#[non_exhaustive]
#[sys_cast(enum(from = Rml_ScrollFlag, repr = i32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScrollFlag {
    None = 0,
    Auto = 1,
}
