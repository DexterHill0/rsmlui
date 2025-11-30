use crate::interfaces::renderer::{IntoRenderInterfacePtr, RenderInterface};

pub struct RendererGl2 {
    pub(crate) raw: *mut rsmlui_sys::render_interface::RenderInterface,
}

impl IntoRenderInterfacePtr for RendererGl2 {
    fn into_ptr(self) -> *mut rsmlui_sys::render_interface::RenderInterface {
        self.raw
    }
}
