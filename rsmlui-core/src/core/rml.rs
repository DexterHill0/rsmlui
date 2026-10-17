use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use drop_tree::{DropCtx, drop_tree};
use rsmlui_macros::rmldoc;
use rsmlui_sys::core;
use rsmlui_sys::file_interface::RmlFileInterface;
use rsmlui_sys::render_interface::RmlRenderInterface;
use rsmlui_sys::system_interface::RmlSystemInterface;

use crate::_private::not_unwind_safe;
use crate::core::backend_handle::BackendHandle;
use crate::core::context::{Context, ContextHandle};
use crate::core::font::{FontFaceOptions, FontFaceSource};
use crate::error::Error;
use crate::interfaces::object::IntoRawInterface;
use crate::math::UVec2;
use crate::utils::conversions::IntoSys;
use crate::utils::private::Uncallable;

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
            core::set_file_interface(ptr::null_mut());
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
        core::set_file_interface(ptr::null_mut());
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

#[rmldoc(file = "api_Rml.md")]
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

    #[rmldoc(name = "Rml::SetSystemInterface")]
    pub fn set_system_interface(
        &self,
        interface: Option<impl IntoRawInterface<RmlSystemInterface>>,
    ) {
        let raw = interface.map_or_else(ptr::null_mut, |itf| itf.into_raw(Uncallable::new()));

        unsafe { core::set_system_interface(raw) }
    }

    #[rmldoc(name = "Rml::SetRenderInterface")]
    pub fn set_render_interface(
        &self,
        interface: Option<impl IntoRawInterface<RmlRenderInterface>>,
    ) {
        let raw = interface.map_or_else(ptr::null_mut, |itf| itf.into_raw(Uncallable::new()));

        unsafe { core::set_render_interface(raw) }
    }

    #[rmldoc(name = "Rml::SetFileInterface")]
    pub fn set_file_interface(&self, interface: Option<impl IntoRawInterface<RmlFileInterface>>) {
        let raw = interface.map_or_else(ptr::null_mut, |itf| itf.into_raw(Uncallable::new()));

        unsafe { core::set_file_interface(raw) }
    }

    #[rmldoc(name = "Rml::Initialise")]
    pub fn initialise(&self) -> Result<(), Error> {
        if is_core_initialized() {
            return Err(Error::AlreadyInitialized);
        }

        if !self.render_interface_installed() {
            return Err(Error::NoRenderInterface);
        }
        if !self.system_interface_installed() {
            return Err(Error::NoSystemInterface);
        }
        if !self.file_interface_installed() {
            return Err(Error::NoFileInterface);
        }
        // TODO: add the following
        // if !self.font_interface_installed() {}

        if !core::initialise() {
            return Err(Error::InitializationFailed);
        }

        IS_INITIALIZED.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// This is also called on `Drop`.
    #[rmldoc(name = "Rml::Shutdown")]
    #[inline]
    pub fn shutdown(self) -> Result<(), Error> {
        drop(self);

        Ok(())
    }

    /// See [`Rml::load_font_face_with_options`] for options.
    #[rmldoc(
        name = "Rml::LoadFontFace",
        refid = "_core_2_core_8h_1a51c118ba84dbfd1cf419f515771988d5"
    )]
    #[inline]
    pub fn load_font_face<F: Into<String>>(
        &self,
        source: FontFaceSource,
        family: F,
    ) -> Result<(), Error> {
        self.load_font_face_with_options(source, family, FontFaceOptions::default())
    }

    #[rmldoc(
        name = "Rml::LoadFontFace",
        refid = "_core_2_core_8h_1a51c118ba84dbfd1cf419f515771988d5"
    )]
    pub fn load_font_face_with_options<F: Into<String>>(
        &self,
        source: FontFaceSource,
        family: F,
        options: FontFaceOptions,
    ) -> Result<(), Error> {
        if !is_core_initialized() {
            return Err(Error::NotInitialized);
        }

        let success = match source {
            FontFaceSource::Path(path) => core::load_font_face_from_file(
                path.into(),
                family.into(),
                options.style.into_sys(),
                options.fallback_face,
                options.weight.into_sys(),
                options.face_index,
            ),
            FontFaceSource::Memory(data) => core::load_font_face_from_memory(
                data,
                family.into(),
                options.style.into_sys(),
                options.fallback_face,
                options.weight.into_sys(),
                options.face_index,
            ),
        };

        if !success {
            return Err(Error::FontFaceLoadFailed);
        }

        Ok(())
    }

    #[rmldoc(name = "Rml::CreateContext")]
    pub fn create_context<N: Into<String>, D: Into<UVec2>>(
        &self,
        name: N,
        dimensions: D,
    ) -> Result<Context, Error> {
        if !is_core_initialized() {
            return Err(Error::NotInitialized);
        }

        let dimensions = dimensions.into();

        let raw = core::create_context(name.into(), dimensions.into_sys());

        if raw.is_null() {
            return Err(Error::ContextCreateFailed);
        }

        Ok(Context::new_with_borrow(
            // Safety: raw is a valid, non-null pointer
            unsafe { ContextHandle::from_raw(raw) },
            self,
        ))
    }

    fn render_interface_installed(&self) -> bool {
        !core::get_render_interface().is_null()
    }

    fn system_interface_installed(&self) -> bool {
        !core::get_system_interface().is_null()
    }

    fn file_interface_installed(&self) -> bool {
        !core::get_file_interface().is_null()
    }
}

// A panic during initialisation could leave the data in a partially-uninitialised state.
// Therefore, it should not be unwind safe.
// `drop-tree` should indirectly cause this impl due to the use of `UnsafeCell` but we're
// being explicit for documentation.
not_unwind_safe!(Rml);
