use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::object::constness::{Constness, Immut, Mut};
use crate::object::ownership::{Native, ObjectOwnership};
use crate::utils::macros::not_send_sync;

/// Identifies a foreign C++ object.
///
/// Implementing this alone does not say whether a handle owns or borrows its object. For a
/// handle to own one ([`Native`]), its tag must also implement [`ForeignDestroy`]. Without a
/// way to free the object, only borrowed handles are representable.
pub trait ForeignObject {
    /// The equivalent foreign type for `Self`.
    type Object;
}

pub type ObjectPtr<O> = *const <O as ForeignObject>::Object;
pub type ObjectMutPtr<O> = *mut <O as ForeignObject>::Object;

/// See [`ForeignObject`].
///
/// Unlike [`ForeignObject`], implementing this means Rust is _able_ to own the resource.
/// Whether a given handle actually does is decided by [`ObjectOwnership`].
///
/// A [`ForeignHandle`] can only be marked [`Native`] (meaning we manage the memory) if
/// this trait is implemented on the tag (the `O` generic of the handle). Just because
/// a tag implements this trait doesn't mean we own the memory. That is decided by
/// whether the handle is [`Native`] or [`Foreign`]. Only if it's [`Native`] will
/// `destroy` be invoked.
///
/// # Safety
///
/// - `destroy` must be the correct way to free a pointer produced by the allocation this
///   tag describes.
/// - `destroy` must be sound to call exactly once.
pub unsafe trait ForeignDestroy: ForeignObject {
    /// Cleanup the object in question.
    ///
    /// # Safety
    ///
    /// - The safety of this method depends on the invariants of the value being
    ///   destroyed. An implementation of this method must uphold those invariants.
    unsafe fn destroy(raw: *mut Self::Object);

    /// Called bedpre [`ForeignDestroy::destroy`]. Use to ensure any runtime invariants
    /// have no been violated by the object being dropped.
    fn assert_droppable(_raw: *mut Self::Object) {}
}

/// Represents any handle to a foreign object. This can be any combination of ownership,
/// constness and provenance. See the trait bounds for more information on these concepts.
///
/// # Provenance
///
/// Provenance simply defines what a given object must outlive for it to be valid. This
/// is represented normally using Rust's lifetimes. Any object where `M = Native` will
/// typically have a `'static` lifetime as the value does not depend on the lifetime of any
/// other object except the program itself, at which point it will be cleaned up at the end
/// of execution. Any object where `M = Foreign` will typically have a lifetime tied to the
/// object that handed out the borrow, so it cannot outlive that object.
///
/// # Aliasing rules
///
/// Two handles to the same object must never have a conflicting [`Constness`], or one
/// could hand out `&'a` references into the object while the other mutates or frees it.
/// Note this is about the object itself, _not_ the ownership. Two handles with a
/// different `M` are equally unsound if they have a conflicting `C`.
///
/// Whether you have to enforce that by hand depends on the relationship of the second handle:
///
/// 1. Child handles: these are obtained from a parent handle. These borrow the parent,
///    through a `self` reference, so the borrow checker already guarantees two conflicting
///    handles can't be live at once, therefore differing constness is fine. This is why a mutable
///    accessor, like a `get_mut` _must_ take `&mut self`.  Getting this wrongcould allow multiple
///    handles to exist together that otherwise shouldn't.
///
/// 2. Sibling handles: similar to child handles, but don't necessarily borrow their direct
///    parent. For example, a [`VariantRef`] from a [`DictionaryRef`]. The variant borrows
///    the `'a` lifetime, _not_ the `'self` lifetime, so it can survive even when the
///    [`DictionaryRef`] is dropped. This makes the two handles siblings, not children.
///    See the **Accessor lifetime rules** section below for more on this case.
///
/// 3. Independent handles: these are created independently, with no parent borrow. For example,
///    a [`ContextHandle`] from `create_context` and another from `get_context` would be two
///    independent handles. These purposefuly don't borrow their parent (that's what `drop-tree`
///    is for), so nothing relates either of them together and the borrow checker can't help.
///    These are the handles the rule above has to be enforced on manually (manually in the
///    sense that care needs to be taken when writing the code and constructing handles to
///    make sure this rule isn't violated).
///
/// By Rust's own rules you can't alias `&mut`, as it carries a uniqueness guarantee the
/// optimiser relies on. Raw pointers do not follow this rule, so two [`Mut`] handles to one
/// object is _not_ UB.
/// Separately, [`ForeignHandle`] being `!Send` and `!Sync` rules out a data race, which
/// _would_ be UB for raw pointers too. The worst outcome is two code paths, for example two event
/// callbacks, both modifying the same object and confusing one another about what the data holds.
///
/// # Accessor lifetime rules
///
/// A handle may return `&'a` references into its object _only_ when `M = Foreign` and
/// `C = Immut`. That combination means the handle itself just a reference to some data
/// it can't outlive. No methods on the handle can invalidate the data it references,
/// therefore any refrenced created from this can take the `'a` life of the parent,
/// making both handles siblings not children. If the original handle is lost, the data
/// it points to is still valid with the lifetime `'a`, so the referenced data is also
/// still valid.
/// For example, `try_get` on a [`DictionaryRef`] returns `&T` with a lifetime belonging to
/// the dictionary object itself. If the [`DictionaryRef`] is dropped, that doesn't mean anything
/// hashappened to the actual dictionary's memory, so the `&T` data is still valid.
///
/// Every other combination must tie references to `&self`, so `&mut self` operations are
/// blocked while any reference is alive. This includes the _read-only_ accessors on a
/// `Mut` handle - in a mutable dictionary, `get` must borrow `&self` so that it conflicts
/// with `get_mut`.
///
/// [`VariantRef`]: crate::containers::variant::VariantRef
/// [`ContextHandle`]: crate::core::context::ContextHandle
/// [`DictionaryRef`]: crate::containers::dictionary::DictionaryRef
pub struct ForeignHandle<'a, O: ForeignObject, M: ObjectOwnership<O> = Native, C: Constness = Immut>
{
    raw: *mut O::Object,
    _lt: PhantomData<&'a ()>,
    _tag: PhantomData<(O, M, C)>,
}

impl<'a, O: ForeignObject, M: ObjectOwnership<O>, C: Constness> Debug
    for ForeignHandle<'a, O, M, C>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForeignHandle")
            .field("raw", &"<foreign object>")
            .finish()
    }
}

not_send_sync!(['a, O: ForeignObject, M: ObjectOwnership<O>, C: Constness] ForeignHandle['a, O, M, C]);

// `retag` is only implemented on `Native` owned objects as anything borrowed (`Foreign`)
// should already be a valid instance.
impl<'a, O1: ForeignDestroy, C: Constness> ForeignHandle<'a, O1, Native, C> {
    /// Change the tag of a given handle.
    ///
    /// # Safety
    ///
    /// - Must not break any invariants the object `O2` requires.
    pub unsafe fn retag<O2>(self) -> ForeignHandle<'a, O2, Native, C>
    where
        O2: ForeignDestroy + ForeignObject<Object = O1::Object>,
    {
        let me = ManuallyDrop::new(self);

        // Safety: `O2` has the same tag type as `O1` (as per the bound)
        // so the cast is valid.
        unsafe { ForeignHandle::<O2, Native, C>::from_mut(me.raw) }
    }
}

impl<'a, O: ForeignObject, M: ObjectOwnership<O>, C: Constness> ForeignHandle<'a, O, M, C> {
    /// Construct a new handle from a raw pointer.
    ///
    /// # Safety
    ///
    /// - The pointer must be a valid, non-null pointer to an instance of [`O::Object`].
    /// - The pointer must be valid for the type `M`<sup>1</sup>.
    ///
    /// While [`ForeignHandle`] can statically verify whether owning the object is
    /// representable, we can't check if the pointer being passed is actually a pointer
    /// valid for the type of `M`<sup>1</sup>.
    ///
    /// 1: That is, a pointer to an object that shouldn't be modified _could_ be passed to
    /// a handle where `M = Mut`. Generally, *const vs *mut should cover most of these cases,
    /// assuming the FFI functions return the right type.
    #[doc(hidden)]
    pub unsafe fn from_raw(raw: C::Ptr<O::Object>) -> Self {
        // Safety: access to the pointer is only *const when `Immut` is the constness
        unsafe { Self::from_mut(C::widen(raw)) }
    }

    /// # Safety
    ///
    /// - The pointer must be a valid, non-null pointer to an instance of [`O::Object`].
    unsafe fn from_mut(raw: *mut O::Object) -> Self {
        Self {
            raw,
            _lt: PhantomData,
            _tag: PhantomData,
        }
    }

    /// Not unsafe but marked as so to discourage usage outside the rsmlui workspace.
    #[doc(hidden)]
    pub unsafe fn as_ptr(&self) -> <Immut as Constness>::Ptr<O::Object> {
        self.raw
    }
}

impl<'a, O: ForeignObject, M: ObjectOwnership<O>> ForeignHandle<'a, O, M, Mut> {
    /// Not unsafe but marked as so to discourage usage outside the rsmlui workspace.
    #[doc(hidden)]
    pub unsafe fn as_mut_ptr(&mut self) -> <Mut as Constness>::Ptr<O::Object> {
        self.raw
    }
}

impl<'a, O: ForeignObject, M: ObjectOwnership<O>, C: Constness> Drop
    for ForeignHandle<'a, O, M, C>
{
    fn drop(&mut self) {
        // Safety: `M::drop` on a `Foreign` value is a noop, and on a `Native`
        // value the drop code is assumed to uphold all invariants when the object
        // is destroyed.
        unsafe { M::on_drop(self.raw) };
    }
}
