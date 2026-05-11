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
/// fires once **all** clones are released, meaning it cannot run until both
/// `Rml` **and** the backend have dropped their handles.
///
/// # Safety model
///
/// There are two invariants to uphold, enforced by two different mechanisms.
///
///  1. **No Rust-held interface pointer survives past its source.**
///
/// [`BorrowedInterface<'a, T>`] carries a lifetime tied to `&self` of whatever
/// produced it (E.G., a backend getter or [`Rml::get_system_interface`]). The
/// borrow checker makes holding one past the source impossible:
///
/// ```rust,compile_fail
/// let itf = backend.get_system_interface(); // borrows backend
/// drop(backend); // error: backend is borrowed
/// ```
///
/// Once passed to [`Rml::set_system_interface`] the [`BorrowedInterface`] is
/// consumed, the borrow ends, and C++ owns the raw pointer from that point on.
///
/// 2. **The backend's C++ resources are never freed while `Rml` is alive.**
///
/// This is enforced at runtime by this type. Both `Rml` and the backend own a
/// clone of the same `Rc<DropClosure>`. The cleanup closure fires only when the
/// refcount reaches zero, meaning both must have released their clones first.
///
/// For a monolithic backend the drop closure calls `backend::shutdown()`. `Rml`'s
/// `Drop` impl calls `core::shutdown()` first (this is to uphold RmlUi's own
/// invariant that interfaces must be non-null during core shutdown),
/// nulls the interface pointers, then releases its handle clone. The drop
/// closure fires when the backend's clone is released.
///
/// For a compositional backend (`Backend<...>` in `rsmlui-backends`), the
/// `BackendInner` (containing multiple [`OwnedInterface<...>`]s of different
/// interfaces, and the window driver) lives inside the drop closure allocation, not
/// in the `Backend` struct itself.
/// Like the monolithic backend, the closure is called, and the data freed, only once
/// both `Rml` and `Backend` have released their handle clones, by which point
/// `core::shutdown()` has already run and the interface pointers are null This
/// satisfies [`OwnedInterface`]'s drop-time `assert_not_registered` check.
///
/// The reason runtime ownership is needed at all: once a [`BorrowedInterface`]
/// is registered with [`Rml`] it is consumed, passing the raw pointer to C++.
/// From that point there are no Rust borrows into the backend; lifetimes
/// cannot track the pointer. In practice the backend stays alive for the entire
/// event loop (`backend.run()` borrows `&mut self` for its duration), so mid-loop
/// the backend cannot be dropped anyway. The real scenario `BackendHandle` guards
/// is the cleanup phase: after the event loop returns, if the backend is dropped
/// before `rml`, the `Rc` mechanism ensures C++ cleanup is still deferred until
/// after `core::shutdown()` completes.
///
/// [`Rml`]: crate::core::core::Rml
/// [`Rml::new`]: crate::core::core::Rml::new
/// [`Rml::get_system_interface`]: crate::core::core::Rml::get_system_interface
/// [`Rml::set_system_interface`]: crate::core::core::Rml::set_system_interface
/// [`BorrowedInterface<'a, T>`]: crate::interfaces::BorrowedInterface
/// [`BorrowedInterface`]: crate::interfaces::BorrowedInterface
/// [`OwnedInterface<...>`]: crate::interfaces::OwnedInterface
/// [`OwnedInterface`]: crate::interfaces::OwnedInterface
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
