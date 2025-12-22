use std::cell::UnsafeCell;

use crate::const_assert_eq;

#[repr(C, packed)]
pub struct InterfaceOpaque {
    _private: [u8; 0],
}

const_assert_eq!(std::mem::size_of::<InterfaceOpaque>(), 0);

pub type InterfaceOpaquePtr = *mut InterfaceOpaque;

pub type ThinInterface<I> = Box<UnsafeCell<I>>;
