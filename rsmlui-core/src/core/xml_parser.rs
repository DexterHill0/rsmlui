use rsmlui_macros::rmldoc;
use rsmlui_sys::xml_node_handler::{RmlXmlNodeHandler, register_xml_node_handler};

use crate::interfaces::object::IntoRawInterface;
use crate::object::constness::NonConst;
use crate::object::foreign::{ForeignHandle, ForeignObject};
use crate::object::ownership::Foreign;
use crate::utils::private::Uncallable;

pub mod tags {
    pub enum TagXmlParser {}
}

impl ForeignObject for tags::TagXmlParser {
    type Object = rsmlui_sys::xml_node_handler::XmlParser;
}

pub type XmlParserHandle = ForeignHandle<'static, tags::TagXmlParser, Foreign, NonConst>;

/// Mutably borrowed version of [`XmlParser`].
#[rmldoc(file = "api_Rml-XMLParser.md", name = "Rml::XMLParser")]
pub type XmlParserRefMut<'a> = ForeignHandle<'a, tags::TagXmlParser, Foreign, NonConst>;

#[rmldoc(file = "api_Rml-XMLParser.md", name = "Rml::XMLParser")]
pub struct XmlParser {
    _private: (),
}

#[rmldoc(file = "api_Rml-XMLParser.md", name = "Rml::XMLParser")]
impl XmlParser {
    #[rmldoc(name = "Rml::XMLParser::RegisterNodeHandler")]
    pub fn register_node_handler<T: AsRef<str>>(
        tag: T,
        handler: impl IntoRawInterface<RmlXmlNodeHandler>,
    ) {
        let raw = handler.into_raw(Uncallable::new());

        unsafe { register_xml_node_handler(tag.as_ref(), raw) }
    }
}
