use sealed::sealed;

/// The following traits represent the "constness" of a given resource.
/// When we borrow an object from C++ (that is, it returns us a pointer to some data)
/// it may not necessarily be sound for that data to be mutated on the rust side.
/// Based on the kind of pointer that is returned (*const or *mut) determines
/// the mutability of the object.
/// The provenance or ownership has no part in whether an object is mutable, as there
/// are instances where foreign (borrowed) objects should still be mutable, such as
/// interfaces.
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
