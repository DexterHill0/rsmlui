pub mod backend;
pub mod renderer;
pub mod system;

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use rsmlui_sys::interfaces::{InterfaceOpaquePtr, ThinInterface};

use crate::interfaces::sealed::Sealed;

pub(crate) mod sealed {
    #[diagnostic::on_unimplemented(
        message = "Interface trait must be implemented on `InterfaceState<{Self}>`",
        label = "This should be `InterfaceState<{Self}>`"
    )]
    #[allow(unused)]
    pub trait Sealed {
        fn class_ptr(&self) -> super::InterfaceOpaquePtr;
    }
}

pub(crate) trait InterfaceMarker {
    type Ptr;
}

pub struct RawInterface<M: InterfaceMarker>(pub(crate) M::Ptr, PhantomData<M>);

impl<M: InterfaceMarker> RawInterface<M> {
    pub(crate) fn new(ptr: M::Ptr) -> Self {
        Self(ptr, PhantomData)
    }
}

pub trait InterfaceInstancer {
    type Output;

    fn instance(self) -> Self::Output
    where
        Self: Sized;
}

pub struct InterfaceHandle<I> {
    pub(crate) value: Pin<ThinInterface<InterfaceState<I>>>,
    // rust needs to handle some clean up by default as interfaces are owned by rust. if they are dropped,
    // the raw box and c++ class instance need to be destroyed. however, rmlui deletes the classes at the end of the program and rust runs drop,
    // which would cause a double free. instead, we hold the pointer in rust so it is rust's responsibility to cleanup everything
    pub(crate) raw: InterfaceOpaquePtr, // this is storing `*mut ThinInterface<InterfaceState<I>>`
    pub(crate) drop: for<'a> unsafe fn(&'a mut Self),
}

impl<I> InterfaceHandle<I> {
    pub(crate) unsafe fn value_mut(&mut self) -> &mut InterfaceState<I> {
        let cell: &mut UnsafeCell<InterfaceState<I>> =
            unsafe { Pin::get_unchecked_mut(self.value.as_mut()) };

        let state: *mut InterfaceState<I> = cell.get();

        unsafe { &mut (*state) }
    }
}

impl<I> Deref for InterfaceHandle<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.value.get()).value }
    }
}

impl<I> DerefMut for InterfaceHandle<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.value.get()).value }
    }
}

impl<T> sealed::Sealed for InterfaceHandle<T> {
    fn class_ptr(&self) -> InterfaceOpaquePtr {
        unsafe { &*self.value.get() }.class_ptr()
    }
}

impl<I> Drop for InterfaceHandle<I> {
    fn drop(&mut self) {
        unsafe { (self.drop)(self) }
    }
}

// TODO: add comments about arbitrary self types needing two wrappers
#[fundamental]
pub struct InterfaceState<I> {
    // user value
    pub(crate) value: I,
    // pointer to interface created in c++
    pub(crate) class_ptr: InterfaceOpaquePtr,
}

impl<I> Deref for InterfaceState<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<I> DerefMut for InterfaceState<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> sealed::Sealed for InterfaceState<T> {
    fn class_ptr(&self) -> InterfaceOpaquePtr {
        self.class_ptr
    }
}

#[repr(transparent)]
pub struct BorrowedInterface<M: InterfaceMarker> {
    pub(crate) raw: M::Ptr,
}

impl<M: InterfaceMarker> BorrowedInterface<M> {
    pub(crate) fn new(ptr: M::Ptr) -> Self {
        Self { raw: ptr }
    }
}
