use std::{
    cell::{RefCell, UnsafeCell},
    sync::LazyLock,
};

use crate::{
    core::context::Context,
    errors::RsmlUiError,
    interfaces::backend::{
        Backend, BackendGuard, BackendOptions, ProcessEventContext, ProcessEventsOptions,
    },
    platforms::win32::PlatformWin32,
    renderers::gl2::RendererGl2,
    utils::{conversions::IntoSys, input::KeyCode},
};

pub struct BackendWin32Gl2 {
    callback: Option<Box<dyn for<'a> FnMut(ProcessEventContext<'a>) -> bool>>,
}

thread_local! {
    static CALLBACK: std::cell::RefCell<
        Option<Box<dyn for<'a> FnMut(ProcessEventContext<'a>) -> bool>>
    > = RefCell::new(None);
}

impl Backend for BackendWin32Gl2 {
    type SystemInterface = PlatformWin32;
    type RenderInterface = RendererGl2;

    fn initialize_with_options<T: Into<String>>(
        window_name: T,
        dimensions: glam::IVec2,
        options: BackendOptions,
    ) -> Result<BackendGuard<Self>, RsmlUiError> {
        let success = rsmlui_sys::backend::initialize(
            window_name.into(),
            dimensions.into_sys(),
            options.allow_resize,
        );

        if !success {
            return Err(RsmlUiError::BackendInitializeFailed);
        }

        return Ok(BackendGuard::new(BackendWin32Gl2 { callback: None }));
    }

    fn shutdown(&mut self) {
        rsmlui_sys::backend::shutdown()
    }

    fn get_system_interface(&mut self) -> Option<Self::SystemInterface> {
        let raw = rsmlui_sys::backend::get_system_interface();

        if raw.is_null() {
            return None;
        }

        Some(PlatformWin32 { raw })
    }

    fn get_render_interface(&mut self) -> Option<Self::RenderInterface> {
        let raw = rsmlui_sys::backend::get_render_interface();

        if raw.is_null() {
            return None;
        }

        Some(RendererGl2 { raw })
    }

    fn set_event_callback(
        &mut self,
        callback: impl FnMut(ProcessEventContext<'_>) -> bool + 'static,
    ) {
        self.callback = Some(Box::new(callback));
    }

    // TODO: improve these impls for downstream users - dont require so much boilerplate / interacting with sys crate
    fn process_events(&mut self, context: &mut Context, options: ProcessEventsOptions) -> bool {
        CALLBACK.with(|slot| {
            *slot.borrow_mut() = self.callback.take();
        });

        fn trampoline(
            ctx: *mut rsmlui_sys::context::Context,
            key: KeyCode,
            key_modifier: i32,
            native_dp_ratio: f32,
            priority: bool,
        ) -> bool {
            let result = CALLBACK.with(|slot| {
                let mut slot = slot.borrow_mut();

                let cb = match slot.as_mut() {
                    Some(cb) => cb,
                    None => return true,
                };

                let mut ctx_wrapped = Context { raw: ctx };

                let event = ProcessEventContext {
                    context: &mut ctx_wrapped,
                    key,
                    key_modifier,
                    native_dp_ratio,
                    priority,
                };

                cb(event)
            });

            result
        }

        let res = unsafe {
            rsmlui_sys::backend::process_events(context.raw, trampoline, options.power_save)
        };

        CALLBACK.with(|slot| {
            self.callback = slot.borrow_mut().take();
        });

        res
    }

    fn request_exit(&mut self) {
        rsmlui_sys::backend::request_exit()
    }

    fn begin_frame(&mut self) {
        rsmlui_sys::backend::begin_frame()
    }

    fn present_frame(&mut self) {
        rsmlui_sys::backend::present_frame()
    }
}
