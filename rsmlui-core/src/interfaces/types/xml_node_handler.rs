use std::mem::transmute;

use rsmlui_macros::rmldoc;
use rsmlui_sys::Rml_XmlDataType;
use rsmlui_sys::utils::c_void;
use rsmlui_sys::xml_node_handler::{
    RmlXmlNodeHandler, RustXmlNodeHandler, XmlNodeHandlerBridge, new_rust_xml_node_handler,
    rust_xml_node_handler_destructor,
};
use sealed::sealed;

use crate::_private::HasOwnedInterface;
use crate::containers::dictionary::DictionaryRef;
use crate::core::xml_parser::XmlParserRefMut;
use crate::interfaces::bridge::BridgedHandle;
use crate::interfaces::object::InterfaceObject;
use crate::types::nodes::XmlDataType;
use crate::utils::conversions::FromSys;
use crate::utils::private::Uncallable;

#[rmldoc(file = "api_Rml-XMLNodeHandler.md", name = "Rml::XMLNodeHandler")]
pub type XmlNodeHandlerObject<T> = InterfaceObject<T, RustXmlNodeHandler>;

#[rmldoc(file = "api_Rml-XMLNodeHandler.md", name = "Rml::XMLNodeHandler")]
pub type XmlNodeHandler<T> = BridgedHandle<T, RustXmlNodeHandler>;

#[sealed]
unsafe impl<T: XmlNodeHandlerBehaviour> crate::interfaces::bridge::BridgeOps<RustXmlNodeHandler>
    for T
{
    fn construct(handle: &mut XmlNodeHandlerObject<Self>) {
        let fat_ptr: *mut dyn XmlNodeHandlerBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_xml_node_handler(meta_raw.cast::<c_void>(), data.cast::<c_void>()) };

        handle.bridge = cpp;
    }

    unsafe fn destroy_bridge(bridge: *mut RustXmlNodeHandler) {
        unsafe { rust_xml_node_handler_destructor(bridge) }
    }

    unsafe fn assert_not_registered(_: *mut RustXmlNodeHandler) {}
}

/// Implement this trait to add behaviour to a custom [`Rml::XMLNodeHandler`].
///
/// All method require implementations.
///
/// Methods receive `self: &mut XmlNodeHandlerObject<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn XmlNodeHandler` trait object.
#[rmldoc(file = "api_Rml-XMLNodeHandler.md", name = "Rml::XMLNodeHandler")]
pub trait XmlNodeHandlerBehaviour {
    // FIXME: make this return Option<ElementRefMut<...>> when transfer of ownership is supported
    #[rmldoc(name = "Rml::XMLNodeHandler::ElementStart")]
    fn element_start(
        self: &mut XmlNodeHandlerObject<Self>,
        parser: XmlParserRefMut<'_>,
        name: &str,
        attributes: DictionaryRef<'_>,
    ) where
        Self: Sized;

    #[rmldoc(name = "Rml::XMLNodeHandler::ElementEnd")]
    fn element_end(
        self: &mut XmlNodeHandlerObject<Self>,
        parser: XmlParserRefMut<'_>,
        name: &str,
    ) -> bool
    where
        Self: Sized;

    #[rmldoc(name = "Rml::XMLNodeHandler::ElementData")]
    fn element_data(
        self: &mut XmlNodeHandlerObject<Self>,
        parser: XmlParserRefMut<'_>,
        data: &str,
        data_type: XmlDataType,
    ) -> bool
    where
        Self: Sized;
}

unsafe impl<T: XmlNodeHandlerBehaviour> XmlNodeHandlerBridge for XmlNodeHandlerObject<T> {
    unsafe fn element_start(
        &mut self,
        parser: *mut rsmlui_sys::xml_node_handler::XmlParser,
        name: &str,
        attributes: &rsmlui_sys::dictionary::Dictionary,
    ) {
        let parser = unsafe { XmlParserRefMut::from_raw(parser) };
        let attributes = unsafe { DictionaryRef::from_raw(attributes as *const _) };

        T::element_start(self, parser, name, attributes);
    }

    unsafe fn element_end(
        &mut self,
        parser: *mut rsmlui_sys::xml_node_handler::XmlParser,
        name: &str,
    ) -> bool {
        let parser = unsafe { XmlParserRefMut::from_raw(parser) };

        T::element_end(self, parser, name)
    }

    unsafe fn element_data(
        &mut self,
        parser: *mut rsmlui_sys::xml_node_handler::XmlParser,
        data: &str,
        data_type: Rml_XmlDataType,
    ) -> bool {
        let parser = unsafe { XmlParserRefMut::from_raw(parser) };

        T::element_data(self, parser, data, FromSys::from_sys(data_type))
    }
}

impl<T: XmlNodeHandlerBehaviour> crate::interfaces::object::IntoRawInterface<RmlXmlNodeHandler>
    for &mut XmlNodeHandler<T>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlXmlNodeHandler {
        self.bridge.cast()
    }
}

impl<T: XmlNodeHandlerBehaviour> HasOwnedInterface<RmlXmlNodeHandler> for T {
    type Owned = XmlNodeHandler<T>;
}
