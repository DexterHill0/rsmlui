use std::panic::{RefUnwindSafe, UnwindSafe};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use drop_tree::{DropCtx, drop_tree};
use glam::IVec2;
use rsmlui_sys::core;
use rsmlui_sys::render_interface::RmlRenderInterface;
use rsmlui_sys::system_interface::RmlSystemInterface;

use crate::core::backend_handle::BackendHandle;
use crate::core::context::Context;
use crate::errors::Error;
use crate::interfaces::{BorrowedInterface, IntoRawInterface};
use crate::utils::conversions::IntoSys;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn is_core_initialized() -> bool {
    IS_INITIALIZED.load(Ordering::SeqCst)
}

fn rml_destructor(_ctx: DropCtx<Rml>) {
    if !is_core_initialized() {
        // Core was not initialised so null any interfaces that were registered before a
        // failed `initialise()` so stale pointers don't corrupt any subsequent `Rml` instance.
        //
        // Not panicking here is intentional as that could cause a double panic and therefore
        // abort and not unwind.
        // It is risky, though, as we run the risk of forgetting to reset some state and leave
        // stale data that could cause UB in future execution.
        unsafe {
            core::set_system_interface(ptr::null_mut());
            core::set_render_interface(ptr::null_mut());
        }

        return;
    }

    // Interfaces must still be registered (non-null & valid) during shutdown as RmlUi uses them for cleanup.
    core::shutdown();

    // After shutdown, null the pointers so any OwnedInterface still in scope
    // can pass their `assert_not_registered` check when they drop.
    unsafe {
        core::set_system_interface(ptr::null_mut());
        core::set_render_interface(ptr::null_mut());
    }

    IS_INITIALIZED.store(false, Ordering::SeqCst);

    // After this function returns, `_ctx` drops, releasing the
    // `BackendHandle`.
    // If the backend already dropped its own clone of the handle,
    // `backend::shutdown()` is called (in the case of a monolithic backend;
    // custom backends are a no-op drop), otherwise it fires when the backend
    // drops.
    // `drop-tree` can't really be used here because it goes across crates,
    // although functionally it's the same idea.
}

#[drop_tree(destructor(rml_destructor))]
pub struct Rml {
    _backend: BackendHandle,
}

impl Rml {
    /// Create an `Rml` instance tied to a given `backend`.
    ///
    /// The handle is obtained from [`Backend::backend_handle`]. Rml stores a
    /// clone of the backend's internal `Rc`. The backend's shutdown closure
    /// only fires once **both** this clone and the backend's own copy are
    /// released. This means `core::shutdown` always completes before the
    /// backend tears down its interfaces, regardless of Rust's drop order.
    ///
    /// [`Backend::backend_handle`]: rsmlui_backends::backend::Backend::backend_handle
    pub fn new(backend: BackendHandle) -> Self {
        Self::new_with_borrow(backend)
    }

    pub fn set_system_interface(
        &self,
        interface: Option<impl IntoRawInterface<RmlSystemInterface>>,
    ) {
        let raw = interface.map_or_else(ptr::null_mut, |itf| itf.into_raw().0);

        unsafe { core::set_system_interface(raw) }
    }

    /// Returns the currently registered system interface, if any.
    ///
    /// The returned `BorrowedInterface` borrows from `&self`, so it cannot outlive this `Rml`
    /// instance. Once the interface is set, the concrete type is erased and it becomes just a
    /// pointer to whatever C++ object was registered, and cannot be safely cast back to a
    /// Rust type.
    pub fn get_system_interface(&self) -> Option<BorrowedInterface<'_, RmlSystemInterface>> {
        let ptr = core::get_system_interface();

        if ptr.is_null() {
            None
        } else {
            Some(BorrowedInterface::new(ptr))
        }
    }

    pub fn set_render_interface(
        &self,
        interface: Option<impl IntoRawInterface<RmlRenderInterface>>,
    ) {
        let raw = interface.map_or_else(ptr::null_mut, |itf| itf.into_raw().0);

        unsafe { core::set_render_interface(raw) }
    }

    /// Returns the currently registered render interface, if any.
    ///
    /// See [`get_system_interface`](Rml::get_system_interface) for lifetime semantics.
    pub fn get_render_interface(&self) -> Option<BorrowedInterface<'_, RmlRenderInterface>> {
        let ptr = core::get_render_interface();

        if ptr.is_null() {
            None
        } else {
            Some(BorrowedInterface::new(ptr))
        }
    }

    pub fn initialise(&self) -> Result<(), Error> {
        if is_core_initialized() {
            return Err(Error::AlreadyInitialized);
        }

        if self.get_render_interface().is_none() {
            return Err(Error::NoRenderInterface);
        }
        if self.get_system_interface().is_none() {
            return Err(Error::NoSystemInterface);
        }
        // TODO: add the following
        // if self.get_font_interface().is_none() {}
        // if self.get_file_interface().is_none() {}

        if !core::initialise() {
            return Err(Error::InitializationFailed);
        }

        IS_INITIALIZED.store(true, Ordering::SeqCst);

        Ok(())
    }

    pub fn shutdown(self) -> Result<(), Error> {
        drop(self);

        Ok(())
    }

    pub fn load_font_face<P: Into<String>>(&self, path: P) -> Result<(), Error> {
        if !is_core_initialized() {
            return Err(Error::NotInitialized);
        }

        let success = core::load_font_face(path.into());

        if !success {
            return Err(Error::FontFaceLoadFailed);
        }

        Ok(())
    }

    pub fn create_context<N: Into<String>, D: Into<IVec2>>(
        &self,
        name: N,
        dimensions: D,
    ) -> Result<Context, Error> {
        if !is_core_initialized() {
            return Err(Error::NotInitialized);
        }

        let context = core::create_context(name.into(), dimensions.into().into_sys());

        if context.is_null() {
            return Err(Error::ContextCreateFailed);
        }

        Ok(Context::new_with_borrow(context, self))
    }
}

// A panic during initialisation could leave the data in a partially-uninitialised state.
// Therefore, it should not be unwind safe.
// `drop-tree` should indirectly cause this impl due to the use of `UnsafeCell` but we're
// being explicit for documentation.
impl !UnwindSafe for Rml {}
impl !RefUnwindSafe for Rml {}
