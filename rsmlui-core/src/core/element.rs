use crate::object::constness::NonConst;
use crate::object::foreign::{ForeignHandle, ForeignObject};
use crate::object::ownership::Foreign;

pub mod tags {
    pub enum TagElement {}
}

impl ForeignObject for tags::TagElement {
    type Object = rsmlui_sys::element::Element;
}

pub type ElementHandle = ForeignHandle<'static, tags::TagElement, Foreign, NonConst>;

// TODO: figure out if this will need `drop-tree` borrowing
// #[rmldoc(file = "api_Rml-Element.md", name = "Rml::Element")]
// #[drop_tree(borrows(crate::core::context::Context))]
pub struct Element {
    handle: ElementHandle,
}

pub type ElementRefMut<'a> = ForeignHandle<'a, tags::TagElement, Foreign, NonConst>;
