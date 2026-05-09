pub mod renderer;
pub mod system;

use std::marker::{PhantomData, PhantomPinned};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use sealed::sealed;

use crate::not_send_sync;

pub struct RawInterface<T>(pub(crate) *mut T, PhantomData<*mut T>);

not_send_sync!([T] RawInterface[T]);

impl<T> RawInterface<T> {
    pub(crate) fn new(ptr: *mut T) -> Self {
        Self(ptr, PhantomData)
    }
}

#[repr(transparent)]
/// A non-owning view of a C++ interface pointer.
///
/// Constructible from a raw pointer via [`BorrowedInterface::new`]. The pointer
/// is not freed on drop as this is just a small, transparent wrapper around the
/// pointer C++ gives us.
#[derive(Copy, Clone)]
pub struct BorrowedInterface<T> {
    pub(crate) raw: *mut T,
}

not_send_sync!([T] BorrowedInterface[T]);

impl<T> BorrowedInterface<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self { raw: ptr }
    }
}

pub trait IntoRawInterface<T> {
    fn into_raw(self) -> RawInterface<T>;
}

impl<T> IntoRawInterface<T> for BorrowedInterface<T> {
    fn into_raw(self) -> RawInterface<T> {
        RawInterface::new(self.raw)
    }
}

/// Heap-allocated data for an interface handle.
///
/// `T` is the user's concrete type. `B` is the C++ bridge object type (e.g.
/// [`RustSystemInterface`] for `SystemInterface`).
///
/// The address of this struct is stored as the `rust_data` pointer inside the C++ bridge object,
/// so it must never move. Callers keep it in a `Pin<Box<InterfaceHandle<T, B>>>` inside
/// [`OwnedInterface`] to guarantee this.
///
/// `InterfaceHandle` is `!Unpin` (via [`PhantomPinned`]) to prevent the compiler from letting
/// safe code move it out of a `Pin`. The `inner` field uses **non-structural pinning**: giving
/// `&mut inner` is always safe because it mutates in place without changing the struct's address.
///
/// [`RustSystemInterface`]: rsmlui_sys::system_interface::RustSystemInterface
/// [`OwnedInterface`]: crate::interfaces::OwnedInterface
pub struct InterfaceHandle<T, B> {
    pub(crate) inner: T,
    pub(crate) bridge: *mut B,
    _pin: PhantomPinned,
    _phantom: PhantomData<B>,
}

not_send_sync!([T, B] InterfaceHandle[T, B]);

impl<T, B> InterfaceHandle<T, B> {
    pub(crate) fn new_pinned(inner: T) -> Pin<Box<Self>> {
        Box::pin(InterfaceHandle {
            inner,
            bridge: std::ptr::null_mut(),
            _pin: PhantomPinned,
            _phantom: PhantomData,
        })
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the user data.
    ///
    /// Safe because `inner` is non-structurally pinned: mutating it in place does not move the
    /// `InterfaceHandle` struct and therefore does not invalidate the C++ pointer to it.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// # Safety
    ///
    /// - `B` must have the same memory layout as [`InterfaceBridgeLayout`].
    ///
    /// [`InterfaceBridgeLayout`]: rsmlui_sys::ffi::interfaces::InterfaceBridgeLayout
    pub(crate) unsafe fn bridge_ptr(&self) -> *mut B {
        self.bridge
    }
}

impl<T, B> Deref for InterfaceHandle<T, B> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner()
    }
}

impl<T, B> DerefMut for InterfaceHandle<T, B> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

/// Allows a Rust interface trait to interface with its respective C++ bridge object.
///
/// It it automatically implemented for all types that implement a supported interface
/// trait (e.g. [`SystemInterface`]). It is a sealed trait and can't be implemented
/// outside `rsmlui-core`.
///
/// [`SystemInterface`]: crate::interfaces::system::SystemInterface
#[sealed(pub(crate))]
pub trait OwnedInterfaceHandle: Sized {
    /// The raw C++ bridge object type, such as `RustSystemInterface`.
    type BridgeObj;

    /// Constructs the c++ bridge and stores the resulting pointer into `handle.bridge`.
    /// Called once, immediately after the handle is heap-pinned in [`OwnedInterface::new`].
    fn init_bridge(handle: &mut InterfaceHandle<Self, Self::BridgeObj>);

    /// Destroys the C++ bridge.
    ///
    /// # Safety
    ///
    /// - Must be called exactly once, before the `InterfaceHandle` is dropped.
    unsafe fn destroy(handle: &mut InterfaceHandle<Self, Self::BridgeObj>);

    /// Asserts if the respective interface is already registered in C++.
    /// Prevents an interface from being dropped if it's still in use.
    fn assert_not_registered(handle: &InterfaceHandle<Self, Self::BridgeObj>);
}

/// Owns a heap-pinned [`InterfaceHandle`] and its associated C++ bridge object.
///
/// Construct via [`OwnedInterface::new`]. The handle address is stable for the value's lifetime
/// because it lives in a `Pin<Box<...>>`. This allows C++ to safely hold a pointer to it.
///
/// # Notes
///
/// Dropping the interface while the interface is registered in RmlUi will cause a panic.
pub struct OwnedInterface<T: OwnedInterfaceHandle> {
    handle: std::pin::Pin<Box<InterfaceHandle<T, T::BridgeObj>>>,
}

not_send_sync!([T: OwnedInterfaceHandle] OwnedInterface[T]);

impl<T: OwnedInterfaceHandle> OwnedInterface<T> {
    pub fn new(interface: T) -> Self {
        let mut handle = InterfaceHandle::new_pinned(interface);

        T::init_bridge(unsafe { handle.as_mut().get_unchecked_mut() });

        OwnedInterface { handle }
    }

    /// Raw pointer to the C++ bridge object.
    pub(crate) fn as_sys_ptr(&self) -> *mut T::BridgeObj {
        unsafe { self.handle.bridge_ptr() }
    }
}

impl<T: OwnedInterfaceHandle> std::ops::Deref for OwnedInterface<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.handle.inner
    }
}

impl<T: OwnedInterfaceHandle> std::ops::DerefMut for OwnedInterface<T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: inner is non-structurally pinned. We only access it in place;
        // the InterfaceHandle address does not change.
        unsafe { self.handle.as_mut().get_unchecked_mut().inner_mut() }
    }
}

impl<T: OwnedInterfaceHandle> Drop for OwnedInterface<T> {
    fn drop(&mut self) {
        T::assert_not_registered(&self.handle);

        // Safety: the above assert will panic if the interface being dropped is actively
        // registered and being used in C++. This will prevent a use-after-free as destroying
        // the object will not prevent RmlUi from still using it.
        unsafe { T::destroy(self.handle.as_mut().get_unchecked_mut()) }
    }
}
