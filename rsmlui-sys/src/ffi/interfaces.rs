use std::cell::UnsafeCell;

#[repr(C)]
pub struct InterfaceOpaque {
    _private: [u8; 0],
}

pub type InterfaceOpaquePtr = *mut InterfaceOpaque;

pub type ThinInterface<I> = Box<UnsafeCell<I>>;
