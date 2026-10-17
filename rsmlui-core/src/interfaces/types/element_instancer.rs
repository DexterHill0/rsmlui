use std::mem::transmute;

use rsmlui_sys::element_instancer::{
    ElementInstancerBridge, RmlElementInstancer, RustElementInstancer, new_rust_element_instancer,
    rust_element_instancer_destructor,
};
use rsmlui_sys::utils::c_void;
use sealed::sealed;

use crate::_private::HasOwnedInterface;
use crate::containers::dictionary::DictionaryRef;
use crate::core::element::ElementRefMut;
use crate::interfaces::bridge::BridgedHandle;
use crate::interfaces::object::InterfaceObject;
use crate::utils::private::Uncallable;

pub type ElementInstancerObject<T> = InterfaceObject<T, RustElementInstancer>;

pub type ElementInstancer<T> = BridgedHandle<T, RustElementInstancer>;

#[sealed]
unsafe impl<T: ElementInstancerBehaviour> crate::interfaces::bridge::BridgeOps<RustElementInstancer>
    for T
{
    fn construct(handle: &mut ElementInstancerObject<Self>) {
        let fat_ptr: *mut dyn ElementInstancerBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_element_instancer(meta_raw.cast::<c_void>(), data.cast::<c_void>()) };

        handle.bridge = cpp;
    }

    unsafe fn destroy_bridge(bridge: *mut RustElementInstancer) {
        unsafe { rust_element_instancer_destructor(bridge) }
    }

    unsafe fn assert_not_registered(_: *mut RustElementInstancer) {}
}

/// Implement this trait to add behaviour to a custom [`Rml::FileInterface`].
///
/// All method require implementations.
///
/// Methods receive `self: &mut FileInterfaceObject<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn FileInterface` trait object.
pub trait ElementInstancerBehaviour {
    fn instance_element(
        self: &mut ElementInstancerObject<Self>,
        parent: Option<ElementRefMut<'_>>,
        tag: &str,
        attributes: DictionaryRef<'_>,
    ) where
        Self: Sized;

    fn release_element(self: &mut ElementInstancerObject<Self>, element: ElementRefMut<'_>)
    where
        Self: Sized;
}

unsafe impl<T: ElementInstancerBehaviour> ElementInstancerBridge for ElementInstancerObject<T> {
    unsafe fn instance_element(
        &mut self,
        parent: *mut rsmlui_sys::element::Element,
        tag: &str,
        attributes: &rsmlui_sys::dictionary::Dictionary,
    ) {
        let parent = if parent.is_null() {
            None
        } else {
            Some(unsafe { ElementRefMut::from_raw(parent) })
        };
        let attributes = unsafe { DictionaryRef::from_raw(attributes as *const _) };

        T::instance_element(self, parent, tag, attributes);
    }

    unsafe fn release_element(&mut self, element: *mut rsmlui_sys::element::Element) {
        T::release_element(self, unsafe { ElementRefMut::from_raw(element) });
    }
}

impl<T: ElementInstancerBehaviour> crate::interfaces::object::IntoRawInterface<RmlElementInstancer>
    for &mut ElementInstancer<T>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlElementInstancer {
        self.bridge.cast()
    }
}

impl<T: ElementInstancerBehaviour> HasOwnedInterface<RmlElementInstancer> for T {
    type Owned = ElementInstancer<T>;
}
