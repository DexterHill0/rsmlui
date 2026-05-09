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
/// fires once **all** clones are released, meaning it cannot run  until both
/// `Rml` **and** the backend have dropped their handles. If only the backend
/// has dropped, its `Drop` impl will not run until `Rml` has dropped its clone
/// of the handle.
///
/// This guarantees that `core::shutdown()` (called from `Rml`'s destructor)
/// always completes before the backend tears down its interfaces, regardless
/// of the order in which the two values are dropped.
///
/// [`Rml`]: crate::core::core::Rml
/// [`Rml::new`]: crate::core::core::Rml::new
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
