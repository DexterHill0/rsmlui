use std::time::Duration;

use glam::IVec2;

use crate::core::context::Context;
use crate::core::core::BACKEND_EVENTS_CALLBACK;
use crate::core::events::{WindowEvent, WindowEventEmitter};
use crate::errors::RsmlUiError;
use crate::interfaces::backend::{BackendOptions, BackendRuntime, MonolithicBackend};
use crate::interfaces::{self};
use crate::utils::conversions::IntoSys;
use crate::utils::input::{KeyCode, KeyModifier};
use crate::utils::raw::Raw;

pub struct BackendWin32Gl2 {
    window_name: String,
    dimensions: IVec2,
    options: BackendOptions,
}

impl interfaces::backend::sealed::Sealed for BackendWin32Gl2 {}

impl BackendWin32Gl2 {
    /// # Panics
    /// Will panic if called more than once.
    pub fn new<N: Into<String>>(
        window_name: N,
        dimensions: glam::IVec2,
        options: BackendOptions,
    ) -> Self {
        Self {
            window_name: window_name.into(),
            dimensions,
            options,
        }
    }
}

impl<T: 'static> BackendRuntime<T> for BackendWin32Gl2 {
    fn initialize(&mut self) -> Result<(), RsmlUiError> {
        let success = rsmlui_sys::backend::initialize(
            self.window_name.clone().into(),
            self.dimensions.into_sys(),
            self.options.allow_resize,
        );

        if !success {
            return Err(RsmlUiError::BackendInitializeFailed);
        }

        let raw_system_interface = rsmlui_sys::backend::get_system_interface();

        if raw_system_interface.is_null() {
            return Err(RsmlUiError::SystemInterfaceFailed);
        }

        unsafe {
            rsmlui_sys::core::set_system_interface(raw_system_interface);
        }

        let raw_render_interface = rsmlui_sys::backend::get_render_interface();

        if raw_render_interface.is_null() {
            return Err(RsmlUiError::RenderInterfaceFailed);
        }

        unsafe {
            rsmlui_sys::core::set_render_interface(raw_render_interface);
        }

        Ok(())
    }

    fn begin_frame(&mut self) {
        rsmlui_sys::backend::begin_frame();
    }

    fn present_frame(&mut self) {
        rsmlui_sys::backend::present_frame();
    }

    fn poll_events(
        &mut self,
        sender: &WindowEventEmitter<T>,
        context: &mut Context,
        _: Duration,
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
            rsmlui_sys::backend::process_events(context.raw(), trampoline, self.options.power_save)
        };

        if !running {
            sender.emit(WindowEvent::ExitRequested)?;
        }

        Ok(())
    }
}

impl<T: 'static> MonolithicBackend<T> for BackendWin32Gl2 {}

impl Drop for BackendWin32Gl2 {
    fn drop(&mut self) {
        rsmlui_sys::backend::shutdown()
    }
}
