use rsmlui_macros::sys_cast;
use rsmlui_sys::Rml_XmlDataType;

#[sys_cast(enum(from = Rml_XmlDataType, repr = i32))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XmlDataType {
    #[sys(Text)]
    Text = 0,
    #[sys(CDATA)]
    CData = 1,
    #[sys(InnerXML)]
    InnerXml = 2,
}
