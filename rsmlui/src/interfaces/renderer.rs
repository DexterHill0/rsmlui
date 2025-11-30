use rsmlui_sys::utils::IntoPtr;

pub trait RenderInterface {}

pub(crate) trait IntoRenderInterfacePtr {
    fn into_ptr(self) -> *mut rsmlui_sys::render_interface::RenderInterface;
}

impl<T: RenderInterface + 'static> IntoRenderInterfacePtr for T {
    fn into_ptr(self) -> *mut rsmlui_sys::render_interface::RenderInterface {
        let adapter = RenderInterfaceExtAdapter { inner: self };

        adapter.into_ptr()
    }
}

pub(crate) struct RenderInterfaceExtAdapter<T: RenderInterface> {
    pub(crate) inner: T,
}

impl<T: RenderInterface> rsmlui_sys::render_interface::RenderInterfaceExt
    for RenderInterfaceExtAdapter<T>
{
}
