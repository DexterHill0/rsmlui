use crate::core::context::Context;
use crate::core::core::BACKEND_EVENTS_CALLBACK;
use crate::core::events::{WindowEvent, WindowEventEmitter};
use crate::errors::RsmlUiError;
use crate::interfaces::BorrowedInterface;
use crate::interfaces::backend::{Backend, BackendOptions};
use crate::platforms::win32::PlatformWin32;
use crate::renderers::gl2::RendererGl2;
use crate::utils::conversions::{FromSys, IntoSys};
use crate::utils::input::{KeyCode, KeyModifier};
use crate::utils::raw::Raw;

pub struct BackendWin32Gl2 {
    system_interface: PlatformWin32,
    render_interface: RendererGl2,
}

impl Backend for BackendWin32Gl2 {
    type RenderInterface = RendererGl2;
    type SystemInterface = PlatformWin32;

    fn initialize_with_options<T: Into<String>>(
        window_name: T,
        dimensions: glam::IVec2,
        options: BackendOptions,
    ) -> Result<Self, RsmlUiError> {
        let success = rsmlui_sys::backend::initialize(
            window_name.into(),
            dimensions.into_sys(),
            options.allow_resize,
        );

        if !success {
            return Err(RsmlUiError::BackendInitializeFailed);
        }

        let raw_system_interface = rsmlui_sys::backend::get_system_interface();

        if raw_system_interface.is_null() {
            return Err(RsmlUiError::SystemInterfaceFailed);
        }

        let raw_render_interface = rsmlui_sys::backend::get_render_interface();

        if raw_render_interface.is_null() {
            return Err(RsmlUiError::RenderInterfaceFailed);
        }

        return Ok(BackendWin32Gl2 {
            system_interface: PlatformWin32(BorrowedInterface::new(raw_system_interface)),
            render_interface: RendererGl2(BorrowedInterface::new(raw_render_interface)),
        });
    }

    fn get_system_interface(&mut self) -> Option<&mut Self::SystemInterface> {
        Some(&mut self.system_interface)
    }

    fn get_render_interface(&mut self) -> Option<&mut Self::RenderInterface> {
        Some(&mut self.render_interface)
    }

    fn process_events<T: 'static>(
        &self,
        context: &mut Context,
        sender: &WindowEventEmitter<T>,
    ) -> Result<(), RsmlUiError> {
        fn trampoline(
            _: *mut rsmlui_sys::context::Context,
            key: KeyCode,
            key_modifier: KeyModifier,
            native_dp_ratio: f32,
            priority: bool,
        ) -> bool {
            let result = BACKEND_EVENTS_CALLBACK.with(|callback| {
                let mut cb = callback.borrow_mut();

                if let Some(cb) = cb.as_mut() {
                    return cb(key, key_modifier, native_dp_ratio, priority);
                }

                true
            });

            result
        }

        let running = unsafe {
            // power saving (reducing polling rate) is handled ourselves
            rsmlui_sys::backend::process_events(context.raw(), trampoline, false)
        };

        if !running {
            sender.emit(WindowEvent::ExitRequested)?;
        }

        Ok(())
    }

    fn request_exit(&self) {
        rsmlui_sys::backend::request_exit()
    }

    fn begin_frame(&self) {
        rsmlui_sys::backend::begin_frame()
    }

    fn present_frame(&self) {
        rsmlui_sys::backend::present_frame()
    }
}

impl Drop for BackendWin32Gl2 {
    fn drop(&mut self) {
        rsmlui_sys::backend::shutdown()
    }
}
