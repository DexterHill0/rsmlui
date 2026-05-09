use std::ops::{Deref, DerefMut};

use rsmlui_core::BackendHandle;
use rsmlui_core::core::context::Context;
use rsmlui_core::interfaces::OwnedInterface;
use rsmlui_core::interfaces::system::SystemInterface;

use crate::error::BackendError;
use crate::window::WindowDriver;

/// The owned fields of a [`Backend`], accessible via `Deref`.
///
/// You should not need to name this type directly. Access should
/// happen through the `Backend` value itself.
pub struct BackendInner<S: SystemInterface, W: WindowDriver> {
    pub system: OwnedInterface<S>,
    pub window: W,
}

/// A compositional backend that owns Rust-implemented interfaces, including a [`WindowDriver`].
/// These interfaces *can* include individual interfaces implemented in C++, such as the
/// Win32 system interface, or GL2 render interface.
///
/// Like a monolithic backend, the caller is responsible for registering the owned
/// interfaces with [`Rml`] and calling [`Rml::initialise`] **before** [`Backend::run`].
/// Interfaces can be registered directly since [`Backend`] derefs to [`BackendInner`]:
///
/// ```no_run
/// struct MySys;
/// impl SystemInterface for MySys {}
///
/// struct MyWin;
/// impl WindowDriver for MyWin {
///     type Error = Infallible;
///     fn process_events(&mut self, _: &Context) -> Result<bool, Self::Error> { Ok(false) }
///     fn begin_frame(&mut self) -> Result<(), Self::Error> { Ok(()) }
///     fn present_frame(&mut self) -> Result<(), Self::Error> { Ok(()) }
///     fn dimensions(&self) -> IVec2 { IVec2::ZERO }
/// }
///
/// let mut backend = Backend::new(OwnedInterface::new(MySys), MyWin);
/// let rml = Rml::new(backend.backend_handle());
///
/// // Register interfaces before initialise.
/// // `rml.initialise()` will fail otherwise.
/// rml.set_system_interface(Some(&backend.system));
/// // ...
/// rml.initialise().unwrap();
///
/// let context = rml.create_context("main", (800, 600)).unwrap();
/// backend.run(&context).unwrap();
/// ```
///
/// # Drop ordering
///
/// The interfaces live inside the [`BackendHandle`] `Rc` allocation, not
/// directly in this struct. They are only destroyed once all handle clones are
/// released (i.e., after both [`Rml`] and [`Backend`] have dropped). This guarantees
/// that `core::shutdown` always completes before any interface is freed,
/// regardless of drop order.
///
/// [`Rml`]: rsmlui_core::core::core::Rml
/// [`Rml::initialise`]: rsmlui_core::core::core::Rml::initialise
pub struct Backend<S: SystemInterface, W: WindowDriver> {
    // Non-owning pointer into the heap allocation owned by `handle`'s closure.
    // Valid for exactly as long as `handle` keeps the Rc alive.
    inner: *mut BackendInner<S, W>,
    handle: BackendHandle,
}

impl<S: SystemInterface + 'static, W: WindowDriver + 'static> Backend<S, W> {
    pub fn new(system: OwnedInterface<S>, window: W) -> Self {
        let raw = Box::into_raw(Box::new(BackendInner { system, window }));

        // Safety: `raw` is boxed above. We give sole ownership
        // to this closure and keep only a non-owning pointer in `inner`.
        let handle = BackendHandle::new(move || unsafe { drop(Box::from_raw(raw)) });

        Self { inner: raw, handle }
    }

    /// Returns a handle to pass to [`Rml::new`].
    ///
    /// `Rml` stores a clone of the internal Rc. The `system` and `window`
    /// allocations are only freed once **both** `Rml` and this `Backend` have
    /// released their clones, ensuring `core::shutdown` always runs first.
    ///
    /// [`Rml::new`]: rsmlui_core::core::core::Rml::new
    pub fn backend_handle(&self) -> BackendHandle {
        self.handle.clone()
    }

    /// Run a single tick (process events, update the context, render, present).
    ///
    /// Returns `Ok(true)` to continue, `Ok(false)` when the window signals exit.
    pub fn tick(&mut self, context: &Context) -> Result<bool, BackendError<W::Error>> {
        let running = self
            .window
            .process_events(context)
            .map_err(BackendError::Window)?;

        if !running {
            return Ok(false);
        }

        context.update()?;
        self.window.begin_frame().map_err(BackendError::Window)?;
        context.render()?;
        self.window.present_frame().map_err(BackendError::Window)?;

        Ok(true)
    }

    /// Run the event loop until the window signals exit.
    ///
    /// Interfaces must already be registered and [`Rml::initialise`] must already have
    /// been called before this.
    ///
    /// [`Rml::initialise`]: rsmlui_core::core::core::Rml::initialise
    pub fn run(&mut self, context: &Context) -> Result<(), BackendError<W::Error>> {
        while self.tick(context)? {}
        Ok(())
    }
}

impl<S: SystemInterface, W: WindowDriver> Deref for Backend<S, W> {
    type Target = BackendInner<S, W>;

    fn deref(&self) -> &BackendInner<S, W> {
        // Safety: `inner` points into the allocation owned by `handle`'s closure.
        // It is valid for the entire lifetime of `Backend` because `handle` keeps
        // the Rc alive. The closure (and its allocation) only fires after `Backend`
        // itself has dropped and released its own handle clone.
        unsafe { &*self.inner }
    }
}

impl<S: SystemInterface, W: WindowDriver> DerefMut for Backend<S, W> {
    fn deref_mut(&mut self) -> &mut BackendInner<S, W> {
        // Safety: same as `Deref`. Exclusive access guaranteed by `&mut self`.
        unsafe { &mut *self.inner }
    }
}
