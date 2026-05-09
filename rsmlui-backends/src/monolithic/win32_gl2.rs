use std::cell::Cell;
use std::convert::Infallible;

use glam::IVec2;
use rsmlui_core::core::context::Context;
use rsmlui_core::errors::Error as CoreError;
use rsmlui_core::interfaces::BorrowedInterface;
use rsmlui_core::types::input::{KeyCode, KeyModifier};
use rsmlui_core::{BackendHandle, IntoSys};
use rsmlui_sys::backend;
use rsmlui_sys::render_interface::RmlRenderInterface;
use rsmlui_sys::system_interface::RmlSystemInterface;

use crate::error::BackendError;
use crate::monolithic::{KeyDownCallback, KeyDownCallbackDyn};

#[derive(Debug, Clone, Copy)]
pub struct Win32Gl2BackendOptions {
    pub allow_resize: bool,
    pub power_save: bool,
}

impl Default for Win32Gl2BackendOptions {
    fn default() -> Self {
        Self {
            allow_resize: true,
            power_save: false,
        }
    }
}

/// Wraps RmlUi's built-in Win32 + OpenGL 2 C++ backend.
///
/// The backend manages its own window, system interface, and render interface internally.
/// Use [`get_system_interface`] and [`get_render_interface`] to obtain borrowed pointers
/// that can be passed to [`Rml::set_system_interface`] / [`Rml::set_render_interface`].
///
/// Setup and call order is functionally the same as the composable [`Backend`].
///
/// ```no_run
/// let mut backend = Win32Gl2Backend::initialize("Demo", (800, 600), Win32Gl2BackendOptions::default()).unwrap();
///
/// let mut rml = Rml::new();
///
/// // Register the handle so `backend::shutdown` is deferred until after `rml` drops.
/// rml.attach_backend(backend.backend_handle());
///
/// rml.set_system_interface(Some(backend.get_system_interface()));
/// // ...
/// rml.initialise().unwrap();
///
/// let context = rml.create_context("main", (800, 600)).unwrap();
///
/// backend.run(&context).unwrap();
/// ```
///
/// [`Backend`]: crate::backend::Backend
/// [`get_system_interface`]: Win32Gl2Backend::get_system_interface
/// [`get_render_interface`]: Win32Gl2Backend::get_render_interface
/// [`Rml::set_system_interface`]: rsmlui_core::core::core::Rml::set_system_interface
/// [`Rml::set_render_interface`]: rsmlui_core::core::core::Rml::set_render_interface
pub struct Win32Gl2Backend {
    handle: BackendHandle,
    key_down_callback: KeyDownCallback,
    options: Win32Gl2BackendOptions,
}

impl Win32Gl2Backend {
    /// Initialise the backend.
    pub fn initialize(
        name: impl Into<String>,
        dimensions: impl Into<IVec2>,
        options: Win32Gl2BackendOptions,
    ) -> Result<Self, BackendError<Infallible>> {
        let success = backend::initialize(
            name.into(),
            dimensions.into().into_sys(),
            options.allow_resize,
        );

        if !success {
            return Err(BackendError::InitializationFailed);
        }

        Ok(Self {
            handle: BackendHandle::new(backend::shutdown),
            key_down_callback: Box::new(noop_key_down),
            options,
        })
    }

    /// Pass this to [`Rml::set_system_interface`](rsmlui_core::core::core::Rml::set_system_interface).
    /// The pointer remains valid until [`shutdown`](Win32Gl2Backend::drop) is called.
    ///
    /// TODO: is it safe to hold the borrowed interface after drop/shutdown?
    pub fn get_system_interface(&self) -> Option<BorrowedInterface<RmlSystemInterface>> {
        let itf = backend::get_system_interface();

        if itf.is_null() {
            return None;
        }

        Some(BorrowedInterface::new(itf))
    }

    /// Pass this to [`Rml::set_render_interface`](rsmlui_core::core::core::Rml::set_render_interface).
    /// The pointer remains valid until [`shutdown`](Win32Gl2Backend::drop) is called.
    ///
    /// TODO: is it safe to hold the borrowed interface after drop/shutdown?
    pub fn get_render_interface(&self) -> Option<BorrowedInterface<RmlRenderInterface>> {
        let itf = backend::get_render_interface();

        if itf.is_null() {
            return None;
        }

        Some(BorrowedInterface::new(itf))
    }

    /// Override the key-down callback used during [`process_events`].
    ///
    /// The default is a no-op.
    ///
    /// [`process_events`]: Win32Gl2Backend::process_events
    pub fn set_key_down_callback(&mut self, callback: KeyDownCallback) {
        self.key_down_callback = callback;
    }

    /// Returns a handle to register with [`Rml::attach_backend`] so that
    /// `backend::shutdown` is deferred until after `Rml` drops.
    ///
    /// [`Rml::attach_backend`]: rsmlui_core::core::core::Rml::attach_backend
    pub fn backend_handle(&self) -> BackendHandle {
        self.handle.clone()
    }

    /// Signal the C++ backend to exit the event loop on the next tick.
    pub fn request_exit(&self) {
        backend::request_exit();
    }

    /// Drive a single frame: process events → update → begin → render → present.
    ///
    /// Returns `Ok(false)` when the window has been closed.
    /// The interfaces must already be registered with [`Rml`] before calling this.
    ///
    /// [`Rml`]: rsmlui_core::core::core::Rml
    pub fn tick(&mut self, context: &Context) -> Result<bool, CoreError> {
        thread_local! {
            static CALLBACK_USER_DATA: Cell<Option<(
                *const Context,
                *mut rsmlui_sys::context::Context,
                *mut KeyDownCallbackDyn,
            )>> = Cell::new(None);
        }

        CALLBACK_USER_DATA.with(|cell| {
            cell.set(Some((
                context as *const _,
                // Safety: the raw pointer is only stored for the duration of `process_events`;
                // it is cleared immediately after and never outlives `context`.
                unsafe { context.as_raw_ptr() },
                self.key_down_callback.as_mut() as *mut _,
            )))
        });

        fn trampoline(
            trampoline_context_ptr: *mut rsmlui_sys::context::Context,
            key: KeyCode,
            modifier: KeyModifier,
            dp_ratio: f32,
            priority: bool,
        ) -> bool {
            CALLBACK_USER_DATA.with(|cell| {
                if let Some((tick_context_ptr, tick_sys_context_ptr, callback_pointer)) = cell.get()
                {
                    assert_eq!(
                        tick_sys_context_ptr, trampoline_context_ptr,
                        "`Context` pointer in callback is not equal \
                        to the one passed to `process_events`"
                    );

                    // Safety: both pointers were stored from a live `&Context` and
                    // `&mut dyn FnMut` at the start of `tick`. The assert above confirms
                    // C++ is calling back with the same context we passed in, so neither
                    // pointer has been invalidated between storage and this call.
                    unsafe {
                        let tick_context = &*tick_context_ptr;

                        return (*callback_pointer)(
                            tick_context,
                            key,
                            modifier,
                            dp_ratio,
                            priority,
                        );
                    }
                }

                false
            })
        }

        // Safety: `context.as_raw_ptr()` is valid for the duration of this call.
        // `trampoline` is a bare fn that reads only from the thread-local set above,
        // which is cleared before this function returns, so no pointer escapes.
        let running = unsafe {
            backend::process_events(context.as_raw_ptr(), trampoline, self.options.power_save)
        };

        CALLBACK_USER_DATA.with(|cell| cell.set(None));

        if !running {
            return Ok(false);
        }

        context.update()?;
        backend::begin_frame();
        context.render()?;
        backend::present_frame();

        Ok(true)
    }

    /// Run the event loop until the window closes.
    ///
    /// See the docs on [`Win32Gl2Backend`] for an example of running the backend.
    pub fn run(&mut self, context: &Context) -> Result<(), CoreError> {
        while self.tick(context)? {}
        Ok(())
    }
}

fn noop_key_down(
    _context: &Context,
    _key: KeyCode,
    _modifier: KeyModifier,
    _dp_ratio: f32,
    _priority: bool,
) -> bool {
    false
}
