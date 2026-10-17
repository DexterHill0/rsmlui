use std::ops::{Deref, DerefMut};

use sealed::sealed;

use crate::_private::not_send_sync;
use crate::utils::private::Uncallable;

/// Heap-allocated data for an interface handle.
///
/// `T` is the user's concrete type. `B` is the C++ bridge object type (e.g.
/// [`RustSystemInterface`] for `SystemInterface`).
///
/// The address of this struct is stored as the `rust_data` pointer inside the C++ bridge object,
/// so it must never move. It is heap allocated with a stable address via [`Box::into_raw`].
/// C++ is only concerned with the address of this struct, but _not_ the fields inside it, so
/// values can be moved in an out of the struct without issue. Of course, any mutations _must_
/// keep the struct in a valid state for when it is later passed back to Rust, and it must not
/// be prematurely freed.
///
/// [`RustSystemInterface`]: rsmlui_sys::system_interface::RustSystemInterface
pub struct InterfaceObject<T, B> {
    pub(crate) inner: T,
    pub(crate) bridge: *mut B,
}

not_send_sync!([T, B] InterfaceObject[T, B]);

impl<T, B> InterfaceObject<T, B> {
    pub(crate) fn new(inner: T) -> Self {
        InterfaceObject {
            inner,
            bridge: std::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the user data.
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut T {
        // Safety: C++ only needs a stable pointer to the struct itself, it does not
        // care what happens to the values inside it.
        &mut self.inner
    }

    /// # Safety
    ///
    /// - `B` must have the same memory layout as [`InterfaceBridgeLayout`].
    ///
    /// [`InterfaceBridgeLayout`]: rsmlui_sys::ffi::interfaces::InterfaceBridgeLayout
    #[inline(always)]
    pub(crate) unsafe fn bridge_ptr(&self) -> *mut B {
        self.bridge
    }
}

impl<T, B> Deref for InterfaceObject<T, B> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner()
    }
}

impl<T, B> DerefMut for InterfaceObject<T, B> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

/// Converts an interface `I` into a raw pointer to the
/// foreign object.
pub trait IntoRawInterface<I> {
    fn into_raw(self, _: Uncallable) -> *mut I;
}

/// Represents an object that can be provded as a given interface.
///
/// Generally will be implemented on whatever [`HasOwnedInterface<I>::Owned`]
/// is set to.
#[sealed(pub(crate))]
pub trait InterfaceSource<I> {}

#[sealed]
impl<T, I> InterfaceSource<I> for T where for<'a> &'a mut T: IntoRawInterface<I> {}

#[doc(hidden)]
pub(crate) mod _private {

    /// Identities, from a given user type, what the owned version of the
    /// interface is.
    ///
    /// For better UX, the backends takes the raw user type that the interface
    /// behaviour trait is implemented on, but that type is the non-owned
    /// non-constructable type, so this trait lets us identity what the owned
    /// version is from the user's type.
    #[doc(hidden)]
    pub trait HasOwnedInterface<I> {
        type Owned;
    }

    #[doc(hidden)]
    pub use rsmlui_sys::file_interface::RmlFileInterface;
    #[doc(hidden)]
    pub use rsmlui_sys::render_interface::RmlRenderInterface;
    #[doc(hidden)]
    pub use rsmlui_sys::system_interface::RmlSystemInterface;
}
