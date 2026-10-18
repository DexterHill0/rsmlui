use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::object::constness::{Const, Constness, NonConst};
use crate::object::ownership::{Native, ObjectOwnership};
use crate::utils::macros::not_send_sync;

/// Identifies a foreign C++ object.
///
/// Implementing this alone does not say whether a handle owns or borrows its object. For a
/// handle to own one ([`Native`]), its tag must also implement [`ForeignDestroy`]. Without a
/// way to free the object, only borrowed handles are representable.
pub trait ForeignObject {
    /// The foreign object represented by the tag (`Self`).
    ///
    /// This will commonly be a pointer to an object allocated by C++.
    /// However, that won't always be true like in the case of interfaces
    /// where they hold a Rust-allocated object but that contains a pointer
    /// to a foreign object.
    ///
    /// In one way or another though there will always be a pointer to a
    /// foreign object contained within whatever `Object` is.
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
    ///
    /// # Safety
    ///
    /// - The pointer may still be aliased by C++, so it should ONLY be read from,
    ///   NEVER mutated.
    unsafe fn assert_droppable(_raw: *mut Self::Object) {}
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
/// Where the latter rule is applied slightly differently is objects where expressing the
/// owner using lifetimes could lead to selt-referential structs. For example, creating a
/// [`Context`] from [`Rml`] would logically create the context with the `'rml` lifetime.
/// However, you may want to store the [`Rml`] and [`Context`] on the same struct, which
/// would not be possible with a lifetime. This is why [`Context`], and other similar objects
/// are `'static` even though they are [`Foreign`]. The `drop-tree` crate uses [`Rc`]'s to
/// model provenance at runtime and ensure objects without explicit lifetime borrows are
/// dropped in the correct order.
///
/// # Aliasing rules
///
/// Handles are allowed to alias freely. The underlying structure in a handle is simply
/// a raw pointer which makes no exclusivity claims on the data. Therefore, two [`NonConst`]
/// handles to the same object is not unsound. Many areas in RmlUi depend on this
/// non-exclusivity.
///
/// The only reason we are able to uphold this claim is because a handle never hands out
/// a Rust reference (I.E `&T` or `&mut T`) into the foreign object.
///
/// Everything also being `!Send` and `!Sync` rules out thread-based data races, which _would_
/// make aliasing raw pointers unsound. The worst outcome of two handles mutating one object
/// is two code paths confusing each other about what the data holds.
///
/// # Reference rules
///
/// A handle must _never_ return a Rust reference into the foreign object, mutable or not.
/// Accessors return owned or [`Copy`] values instead.
///
/// A `&mut T` reference is simply invalid to hand out as the memory is not ours and allowing
/// us to mutate it could lead to numerous invariants of the foreign code being violated.
///
/// A `&T` is different. It promises the bytes it points at don't change for its lifetime.
/// However, nothing about the handle that issues the reference can truly guarantee that.
/// For example, the owner of the issuing handle could mutate it, and as far as Rust is concerned,
/// the two handles are unrelated values, so the borrow checker sees no issues.
///
/// Unfortunately, tying `&T` to `&self` (via `&'_ T`) is _not_ sufficient, because every handle
/// between the owner and the reference would also have to reborrow, rather than propagate `'a`.
/// Anywhere that fails to do that would open up the same hole above that the borrow checker
/// would not detct.
///
/// The above applies to foreign data only. For example, [`InterfaceObject`] lives in a [`Box`],
/// so the `&mut` handed out by [`BridgedHandle`]'s [`DerefMut`] is an ordinary reference into
/// Rust-owned data, so it is safe to hand out.
///
/// # What `'a` means
///
/// `'a` carries the validity of the object, AKA how long it lives for. It has no control over
/// mutation, that is what the reference rules are for.
///
/// `'a` names whatever keeps the object alive, _not_ the handle it was created from.
/// For example, [`Element`]'s are owned by an [`ElementDocument`]. When borrowing a child
/// [`Element`] from a parent, the returned handle is given the `'a` from the [`ElementDocument`]
/// _not_ the `'a` from the parent element, as they all live for as long as the same document.
/// Reborrowing from the parent element would still be sound, but would make
/// `parent.append_child(child)` unwritable.
///
/// [`NonConst`]: crate::object::constness::NonConst
/// [`InterfaceObject`]: crate::interfaces::object::InterfaceObject
/// [`BridgedHandle`]: crate::interfaces::bridge::BridgedHandle
/// [`DerefMut`]: std::ops::DerefMut
/// [`Element`]: crate::core::element::Element
/// [`ElementDocument`]: crate::core::element_document::ElementDocument
/// [`Context`]: crate::core::context::Context
/// [`Rml`]: crate::core::rml::Rml
/// [`Rc`]: std::rc::Rc
/// [`Foreign`]: crate::object::ownership::Foreign
pub struct ForeignHandle<'a, O: ForeignObject, M: ObjectOwnership<O> = Native, C: Constness = Const>
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
    /// valid for the type of `C`<sup>1</sup>.
    ///
    /// 1: That is, a pointer to an object that shouldn't be modified _could_ be passed to
    /// a handle where `C = NonConst`. Generally, *const vs *mut should cover most of these cases,
    /// assuming the FFI functions return the right type.
    #[doc(hidden)]
    pub unsafe fn from_raw(raw: C::Ptr<O::Object>) -> Self {
        // Safety: access to the pointer is only *const when `Const` is the constness
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
    pub unsafe fn as_ptr(&self) -> <Const as Constness>::Ptr<O::Object> {
        self.raw
    }
}

impl<'a, O: ForeignObject, M: ObjectOwnership<O>> ForeignHandle<'a, O, M, NonConst> {
    /// Not unsafe but marked as so to discourage usage outside the rsmlui workspace.
    #[doc(hidden)]
    pub unsafe fn as_mut_ptr(&mut self) -> <NonConst as Constness>::Ptr<O::Object> {
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
