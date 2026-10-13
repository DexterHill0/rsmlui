use sealed::sealed;

/// The following traits represent the "constness" of a given resource.
/// When we borrow an object from C++ (that is, it returns us a pointer to some data)
/// it may not necessarily be sound for that data to be mutated on the rust side.
/// Based on the kind of pointer that is returned (*const or *mut) determines
/// the mutability of the object.
/// The provenance or ownership has no part in whether an object is mutable, as there
/// are instances where foreign (borrowed) objects should still be mutable, such as
/// interfaces.
///
/// # Receiver types
///
/// Pick a receiver by mirroring the C++ member function's `const` qualifier, not the
/// pointer's. A non-const member function is RmlUi stating it mutates `*this`, so it
/// takes `&mut self`. In contrast, a `const` one takes only `&self`.
/// A general rule of thumb: `const` methods go in the `C: Constness` block and use
/// `as_ptr`, non-const methods go in the `Mut` block and use `as_mut_ptr`.
///
/// While mixing them up isn't ideal, there won't be any unsound behaviour as the
/// `as_mut_ptr` won't be callable from a `Immut` handle, even if you accidentally
/// write the reciever as `&mut self`, it will just simply mean the user will require
/// mut access to the handle uneccessarily.
#[sealed]
pub trait Constness {
    type Ptr<T>;

    fn widen<T>(ptr: Self::Ptr<T>) -> *mut T;
}

/// The resource is immutabe. It can only be constructed from a *const pointer, and can only return a *const pointer.
pub enum Immut {}

/// The resource is mutable. It can only be constructed from a *mut pointer, and can return both *const and *mut pointers.
pub enum Mut {}

#[sealed]
impl Constness for Immut {
    type Ptr<T> = *const T;

    fn widen<T>(ptr: *const T) -> *mut T {
        ptr.cast_mut()
    }
}

#[sealed]
impl Constness for Mut {
    type Ptr<T> = *mut T;

    fn widen<T>(ptr: *mut T) -> *mut T {
        ptr
    }
}
