use std::cell::Cell;
use std::convert::Infallible;

use rsmlui_core::_private::{FromSys, IntoSys, not_send_sync, not_unwind_safe};
use rsmlui_core::core::context::Context;
use rsmlui_core::error::Error as CoreError;
use rsmlui_core::math::IVec2;
use rsmlui_core::types::input::{KeyCode, KeyModifier};
use rsmlui_core::{BackendHandle, Rml};
use rsmlui_sys::{Rml_Input_KeyIdentifier, Rml_Input_KeyModifier, backend};

use crate::error::BackendError;
use crate::interfaces::default::FileDefault;
use crate::interfaces::gl2::RendererGl2Ref;
use crate::interfaces::glfw::SystemGlfwRef;
use crate::monolithic::{BackendOptions, KeyDownCallback, KeyDownCallbackDyn};

/// Wraps RmlUi's built-in GLFW + OpenGL 2 C++ backend.
///
/// The backend manages its own window, system interface, and render interface internally.
/// Use [`get_system_interface`] and [`get_render_interface`] to obtain borrowed pointers
/// that can be passed to [`Rml::set_system_interface`] / [`Rml::set_render_interface`].
///
/// Setup and call order is functionally the same as the composable [`Backend`].
///
/// ```no_run
/// let mut backend = GlfwGl2Backend::initialize("Demo", (800, 600), BackendOptions::default()).unwrap();
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
/// [`get_system_interface`]: GlfwGl2Backend::get_system_interface
/// [`get_render_interface`]: GlfwGl2Backend::get_render_interface
/// [`Rml::set_system_interface`]: rsmlui_core::core::core::Rml::set_system_interface
/// [`Rml::set_render_interface`]: rsmlui_core::core::core::Rml::set_render_interface
pub struct GlfwGl2Backend {
    handle: BackendHandle,
    key_down_callback: KeyDownCallback,
    options: BackendOptions,
    file_interface: FileDefault,
}

not_send_sync!(GlfwGl2Backend);
not_unwind_safe!(GlfwGl2Backend);

impl GlfwGl2Backend {
    /// Initialise the backend.
    pub fn initialize(
        name: impl Into<String>,
        dimensions: impl Into<IVec2>,
        options: BackendOptions,
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
            file_interface: FileDefault::new(),
        })
    }

    /// Installs the interfaces into the [`Rml`] instance.
    ///
    /// Installs the [`FileDefault`] interface as the file interface.
    ///
    /// [`FileDefault`]: crate::interfaces::default::FileDefault
    pub fn install_interfaces(&mut self, rml: &mut Rml) {
        let system = backend::get_system_interface();

        if !system.is_null() {
            let mut itf = unsafe { SystemGlfwRef::from_raw(system) };

            rml.set_system_interface(Some(&mut itf));
        }

        let render = backend::get_render_interface();

        if !render.is_null() {
            let mut itf = unsafe { RendererGl2Ref::from_raw(render) };

            rml.set_render_interface(Some(&mut itf));
        }

        let file = &mut self.file_interface;

        rml.set_file_interface(Some(file));
    }

    /// Override the key-down callback used during [`process_events`].
    ///
    /// The default is a no-op.
    ///
    /// [`process_events`]: GlfwGl2Backend::process_events
    pub fn set_key_down_callback(&mut self, callback: KeyDownCallback) {
        self.key_down_callback = callback;
    }

    /// Returns a handle to pass to [`Rml::new`](rsmlui_core::core::core::Rml::new) so that
    /// correct clean-up order for all resources is preserved.
    pub fn backend_handle(&self) -> BackendHandle {
        self.handle.clone()
    }

    /// Signal the C++ backend to exit the event loop on the next tick.
    pub fn request_exit(&self) {
        backend::request_exit();
    }

    /// Run a single frame end-to-end.
    ///
    /// Returns `Ok(false)` when the window has been closed.
    /// The interfaces must already be registered with [`Rml`] before calling this.
    ///
    /// [`Rml`]: rsmlui_core::core::core::Rml
    pub fn tick(&mut self, context: &mut Context) -> Result<bool, CoreError> {
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
                unsafe { context.as_mut_ptr() },
                self.key_down_callback.as_mut() as *mut _,
            )))
        });

        fn trampoline(
            trampoline_context_ptr: *mut rsmlui_sys::context::Context,
            key: Rml_Input_KeyIdentifier,
            modifier: Rml_Input_KeyModifier,
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
                            FromSys::from_sys(key),
                            FromSys::from_sys(modifier),
                            dp_ratio,
                            priority,
                        );
                    }
                }

                false
            })
        }

        // Safety: `context.as_mut_ptr()` is valid for the duration of this call.
        // `trampoline` is a bare fn that reads only from the thread-local set above,
        // which is cleared before this function returns, so no pointer escapes.
        let running = unsafe {
            backend::process_events(context.as_mut_ptr(), trampoline, self.options.power_save)
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
    /// See the docs on [`GlfwGl2Backend`] for an example of running the backend.
    pub fn run(&mut self, context: &mut Context) -> Result<(), CoreError> {
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
