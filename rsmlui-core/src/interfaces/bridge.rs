use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use sealed::sealed;

use crate::interfaces::object::InterfaceObject;
use crate::object::constness::{Constness, Mut};
use crate::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject};
use crate::object::ownership::{Native, ObjectOwnership};

/// All interface tags represent an [`InterfaceObject`] that contains `T`,
/// the user's interface implementation, and `B`, the C++ bridge struct
/// that contains the neccessary information to invoke the interface trait
/// methods.
///
/// This struct acts as as wrapper for interface tags so each one doesn't
/// need to duplicate the `ForeignObject` and `ForeignDestroy` implementations.
pub struct BridgedTag<T, B>(PhantomData<(T, B)>);

impl<T, B> ForeignObject for BridgedTag<T, B> {
    type Object = InterfaceObject<T, B>;
}

/// # Safety
///
/// - See [`BridgeOps::destroy_bridge`] and [`BridgeOps::assert_not_registered`].
#[sealed(pub(crate))]
pub unsafe trait BridgeOps<B>: Sized {
    fn construct(handle: &mut InterfaceObject<Self, B>);

    /// # Safety
    ///
    /// - `destroy_bridge` must be the correct destructor for the bridge [`BridgeOps::construct`] built.
    unsafe fn destroy_bridge(bridge: *mut B);

    /// # Safety
    ///
    /// - The pointer may still be aliased by C++, so it should ONLY be read from,
    ///   NEVER mutated.
    unsafe fn assert_not_registered(bridge: *mut B);
}

unsafe impl<T: BridgeOps<B>, B> ForeignDestroy for BridgedTag<T, B> {
    unsafe fn destroy(raw: *mut Self::Object) {
        // Safety: an impl of BridgeOps assumes `destroy_bridge` is the correct destructor
        unsafe { T::destroy_bridge((*raw).bridge_ptr()) };

        // Safety: `InterfaaceObject::new` calls `Box::into_raw`, so this is a valid
        // box pointer we must reclaim
        drop(unsafe { Box::from_raw(raw) });
    }

    unsafe fn assert_droppable(raw: *mut Self::Object) {
        // Safety: no invariants but the function may derefence a raw pointer
        unsafe { T::assert_not_registered((*raw).bridge_ptr()) }
    }
}

/// Wrapper around a [`ForeignHandle`] with a [`BridgedTag`].
///
/// `T` is the user's interface implementation and `B` is the C++ bridge struct.
pub type BridgedHandle<T, B> = ForeignHandle<'static, BridgedTag<T, B>, Native, Mut>;

impl<T: BridgeOps<B>, B> BridgedHandle<T, B> {
    pub fn new(interface: T) -> Self {
        let mut handle = Box::new(InterfaceObject::new(interface));

        T::construct(&mut handle);

        // Safety: the memory allocated by this function is freed in `ForeignDestroy::destroy`
        unsafe { BridgedHandle::from_raw(Box::into_raw(handle)) }
    }
}

impl<T, B, M: ObjectOwnership<BridgedTag<T, B>>, C: Constness> Deref
    for ForeignHandle<'_, BridgedTag<T, B>, M, C>
{
    type Target = InterfaceObject<T, B>;

    fn deref(&self) -> &Self::Target {
        // Safety: the pointer is the `Box` allocated in `Bridged::new` and lives
        // as long as this handle
        unsafe { &*self.as_ptr() }
    }
}

impl<T, B, M: ObjectOwnership<BridgedTag<T, B>>> DerefMut
    for ForeignHandle<'_, BridgedTag<T, B>, M, Mut>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
