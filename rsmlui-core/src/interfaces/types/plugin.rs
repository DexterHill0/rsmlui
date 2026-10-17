use std::mem::transmute;

use rsmlui_macros::rmldoc;
use rsmlui_sys::core::unregister_plugin;
use rsmlui_sys::plugin_interface::{
    PluginInterfaceBridge, RmlPlugin, RustPluginInterface, new_rust_plugin_interface,
    rust_plugin_interface_destructor,
};
use rsmlui_sys::utils::c_void;
use sealed::sealed;

use crate::_private::HasOwnedInterface;
use crate::core::context::ContextRefMut;
use crate::core::element_document::ElementDocumentRefMut;
use crate::interfaces::bridge::BridgedHandle;
use crate::interfaces::object::InterfaceObject;
use crate::utils::private::Uncallable;

pub type PluginInterfaceObject<T> = InterfaceObject<T, RustPluginInterface>;

/// A custom [`Rml::PluginInterface`].
///
/// [`Rml::PluginInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/file.html
pub type PluginInterface<T> = BridgedHandle<T, RustPluginInterface>;

#[sealed]
unsafe impl<T: PluginInterfaceBehaviour> crate::interfaces::bridge::BridgeOps<RustPluginInterface>
    for T
{
    fn construct(handle: &mut PluginInterfaceObject<Self>) {
        // The fat pointer data component is the address of the heap-allocated `InterfaceObject`.
        // That address is stable for the lifetime of the `BridgedHandle`.
        let fat_ptr: *mut dyn PluginInterfaceBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_plugin_interface(meta_raw.cast::<c_void>(), data.cast::<c_void>()) };

        handle.bridge = cpp;
    }

    unsafe fn destroy_bridge(bridge: *mut RustPluginInterface) {
        // Safey: plugins cans safely be registered and unregistered
        // at any time
        unsafe {
            unregister_plugin(bridge.cast());
        }

        // Safety: `bridge` is a valid, non-null pointer
        unsafe { rust_plugin_interface_destructor(bridge) }
    }

    unsafe fn assert_not_registered(_: *mut RustPluginInterface) {
        // Always safe to drop, we unregister before running the destructor
    }
}

/// Implement this trait to add behaviour ot a custom [`Rml::Plugin`].
///
/// All methods have default implementations.
///
/// Methods receive `self: &mut PluginInterfaceObject<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn PluginInterface` trait object.
///
/// [`Rml::Plugin`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/plugins.html
#[allow(unused)]
#[rmldoc(file = "api_Rml-Plugin.md", name = "Rml::Plugin")]
pub trait PluginInterfaceBehaviour {
    // NOTE: This trait's default method impls don't call back out to the C++
    // class, like we do in other interfaces. I think this is okay, since a plugin
    // will never have any default behaviour in its methods so, to save on code,
    // the defaults are defined in this trait rather than calling C++ unnecessarily.

    #[rmldoc(name = "Rml::Plugin::OnInitialise")]
    fn on_initialize(self: &mut PluginInterfaceObject<Self>)
    where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnShutdown")]
    fn on_shutdown(self: &mut PluginInterfaceObject<Self>)
    where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnDocumentOpen")]
    fn on_document_open(
        self: &mut PluginInterfaceObject<Self>,
        context: ContextRefMut<'_>,
        document_path: &str,
    ) where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnDocumentLoad")]
    fn on_document_load(self: &mut PluginInterfaceObject<Self>, document: ElementDocumentRefMut<'_>)
    where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnDocumentUnload")]
    fn on_document_unload(
        self: &mut PluginInterfaceObject<Self>,
        document: ElementDocumentRefMut<'_>,
    ) where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnContextCreate")]
    fn on_context_create(self: &mut PluginInterfaceObject<Self>, context: ContextRefMut<'_>)
    where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnContextDestroy")]
    fn on_context_destroy(self: &mut PluginInterfaceObject<Self>, context: ContextRefMut<'_>)
    where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnDataModelCreate")]
    fn on_data_model_create(
        self: &mut PluginInterfaceObject<Self>,
        context: ContextRefMut<'_>,
        name: &str,
    ) where
        Self: Sized,
    {
    }

    #[rmldoc(name = "Rml::Plugin::OnDataModelDestroy")]
    fn on_data_model_destroy(
        self: &mut PluginInterfaceObject<Self>,
        context: ContextRefMut<'_>,
        name: &str,
    ) where
        Self: Sized,
    {
    }
}

// The sys crate uses `PluginInterfaceBridge` as the dispatch from C++ to Rust.
// This impl forwards each call to the user's `PluginInterface` implementation.
unsafe impl<T: PluginInterfaceBehaviour> PluginInterfaceBridge for PluginInterfaceObject<T> {
    unsafe fn on_initialize(&mut self) {
        T::on_initialize(self);
    }

    unsafe fn on_shutdown(&mut self) {
        T::on_shutdown(self);
    }

    unsafe fn on_document_open(
        &mut self,
        context: *mut rsmlui_sys::context::Context,
        document_path: &str,
    ) {
        T::on_document_open(
            self,
            unsafe { ContextRefMut::from_raw(context) },
            document_path,
        );
    }

    unsafe fn on_document_load(
        &mut self,
        document: *mut rsmlui_sys::element_document::ElementDocument,
    ) {
        T::on_document_load(self, unsafe { ElementDocumentRefMut::from_raw(document) });
    }

    unsafe fn on_document_unload(
        &mut self,
        document: *mut rsmlui_sys::element_document::ElementDocument,
    ) {
        T::on_document_unload(self, unsafe { ElementDocumentRefMut::from_raw(document) });
    }

    unsafe fn on_context_create(&mut self, context: *mut rsmlui_sys::context::Context) {
        T::on_context_create(self, unsafe { ContextRefMut::from_raw(context) });
    }

    unsafe fn on_context_destroy(&mut self, context: *mut rsmlui_sys::context::Context) {
        T::on_context_destroy(self, unsafe { ContextRefMut::from_raw(context) });
    }

    unsafe fn on_data_model_create(
        &mut self,
        context: *mut rsmlui_sys::context::Context,
        name: &str,
    ) {
        T::on_data_model_create(self, unsafe { ContextRefMut::from_raw(context) }, name);
    }

    unsafe fn on_data_model_destroy(
        &mut self,
        context: *mut rsmlui_sys::context::Context,
        name: &str,
    ) {
        T::on_data_model_destroy(self, unsafe { ContextRefMut::from_raw(context) }, name);
    }
}

// Using `RmlPlugin` so it's consistent with existing C++ file interfaces.
// `RustPluginInterface` is simply a subclass of `RmlPlugin` so it represents
// the same kind of object.
impl<T: PluginInterfaceBehaviour> crate::interfaces::object::IntoRawInterface<RmlPlugin>
    for &mut PluginInterface<T>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlPlugin {
        self.bridge.cast()
    }
}

impl<T: PluginInterfaceBehaviour> HasOwnedInterface<RmlPlugin> for T {
    type Owned = PluginInterface<T>;
}
