use std::rc::Rc;

// Separate so that `Drop` is on the inner `Rc`-ed value
struct DropClosure(Option<Box<dyn FnOnce()>>);

impl Drop for DropClosure {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

/// Defers a backend's shutdown until after [`Rml`] has shut down.
///
/// A backend creates one handle via [`BackendHandle::new`], passing it a cleanup
/// function. It then gives a clone to `Rml` via [`Rml::new`]. The cleanup only
/// fires once all clones are released, meaning it cannot run until both
/// [`Rml`] _and_ the backend have dropped their handles.
///
/// # Safety
///
/// There are two invariants to uphold:
///
///  1. No foreign interface survives past its source.
///
/// The [`ForeignHandle`] struct, when implemented with the [`Foreign`] ownership
/// model, requires a lifetime that borrows from its source, so the interface
/// can't outlive that.
///
/// ```rust,compile_fail
/// let itf = backend.get_system_interface(); // borrows backend
/// drop(backend); // error: backend is borrowed
/// ```
///
/// However, `&mut` on a [`Foreign`] object does not mean exclusive access.
/// Interfaces, for example, may be installed into RmlUi while simultaneously
/// maintaining `&mut` access in Rust. The single-threaded nature of RmlUi means
/// this can, at worst, only lead to logic issues, not UB.
///
/// 2. The foreign objbects are never freed while [`Rml`] is alive.
///
/// This is upheld at runtime by this type. Both [`Rml`] and the backend own a
/// clone of the same `Rc<DropClosure>`. The cleanup closure fires only when the
/// refcount reaches zero, meaning both must have released their clones first.
///
/// For both a monolithic and compositional backend, [`Rml`]'s [`Drop`] impl
/// calls `core::shutdown()` first (this is to uphold RmlUi's own invariant that
/// interfaces _must_ be non-null during shutdown), nulls the interface pointers,
/// then releases its handle clone.
///
/// For a monolithic backend: after the clone is dropped, if the refcount is zero
/// the drop closure calls `backend::shutdown()`.
///
/// For a compositional backend (`Backend<...>` in `rsmlui-backends`): the
/// `BackendInner` lives inside the drop closure allocation, not
/// in the `Backend` struct itself (the struct only stores a pointer to allocation
/// for [`Deref`] and [`DerefMut`]). When the clone is dropped and the refcount is
/// zero, the drop closure is called and the data is freed. Because the closure
/// always runs after `core::shutdown`, the interface pointers have already been
/// set to null, which satisfies [`BridgedTag`]'s drop-time
/// `assert_not_registered` check.
///
/// [`Rml`]: crate::core::core::Rml
/// [`Rml::new`]: crate::core::core::Rml::new
/// [`Rml::get_system_interface`]: crate::core::core::Rml::get_system_interface
/// [`Rml::set_system_interface`]: crate::core::core::Rml::set_system_interface
/// [`ForeignHandle`]: crate::object::foreign::ForeignHandle
/// [`Foreign`]: crate::object::ownership::Foreign
/// [`BridgedTag`]: crate::interfaces::bridge::BridgedTag
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
pub struct BackendHandle(Rc<DropClosure>);

impl BackendHandle {
    /// Create a new handle whose cleanup closure will be called once all
    /// clones of this handle have been dropped.
    ///
    /// Used to uphold the invariant of Rml that explicitely states a interfaces
    /// must not be destroyed until `core::shutdown` has completed (AKA, the
    /// `Drop` impl on [`Rml`]).
    ///
    /// [`Rml`]: crate::core::core::Rml
    pub fn new(on_drop: impl FnOnce() + 'static) -> Self {
        Self(Rc::new(DropClosure(Some(Box::new(on_drop)))))
    }
}

impl Clone for BackendHandle {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
