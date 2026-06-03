pub trait FromSys<T> {
    fn from_sys(value: T) -> Self;
}

pub trait IntoSys<T> {
    fn into_sys(self) -> T;
}

/// Validates that the memory at a raw sys pointer can be safely converted to `Self`.
///
/// Takes `*const Self::Sys` so the incoming FFI bytes are never turned into a Rust
/// reference (which would be UB for constructs like enums with invalid discriminants).
///
/// # Safety
///
/// - `ptr` must be non-null, valid for reads, and aligned for `Self::Sys`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as a `sys_cast` struct field",
    label = "`{Self}` does not implement `SysCast`",
    note = "annotate `{Self}` with `#[sys_cast(...)]`, or manually add `unsafe impl SysCast for {Self}` if it has no rsmlui-sys counterpart"
)]
pub unsafe trait SysCast: Sized {
    type Sys: Sized;

    /// Validates that the bytes at `ptr` represent a valid `Self`.
    /// Used when converting `sys -> core` at an FFI boundary.
    unsafe fn validate(ptr: *const Self::Sys) -> bool {
        let _ = ptr;
        true
    }

    /// Validates that `value` can be safely represented as `Self::Sys`.
    /// Used when converting `core -> sys`. Defaults to always valid.
    fn validate_core(value: &Self) -> bool {
        let _ = value;
        true
    }
}

macro_rules! impl_sys_cast_primitive {
    ($($t:ty),* $(,)?) => {
        $(unsafe impl SysCast for $t { type Sys = $t; })*
    };
}

impl_sys_cast_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);

unsafe impl SysCast for bool {
    type Sys = bool;

    unsafe fn validate(ptr: *const Self::Sys) -> bool {
        let byte = unsafe { *(ptr as *const u8) };

        byte == 0 || byte == 1
    }
}

/// Validate and convert a sys value to its Rust counterpart.
///
/// Panics if `T::validate` returns false (i.e. the bytes don't represent a valid `T`).
///
/// # Safety
///
/// - `T` must have the same size and alignment as `T::Sys`. This is guaranteed for any
///   type annotated with `#[sys_cast(...)]` via the compile-time assertions the macro emits.
#[allow(unused)]
pub fn sys_cast<T: SysCast>(value: T::Sys) -> T {
    assert!(
        unsafe { T::validate(&raw const value) },
        "sys_cast: invalid sys value for {}",
        std::any::type_name::<T>(),
    );

    // Safety: same size/align enforced by `sys_cast` macro const assertions.
    // Validity also checked above.
    unsafe { std::mem::transmute_copy(&value) }
}

/// Validate and convert a core value to its sys counterpart.
///
/// Panics if `T::validate_core` returns false (i.e. the value is out of range for `T::Sys`).
///
/// # Safety
///
/// - `T` must have the same size and alignment as `T::Sys`. This is guaranteed for any
///   type annotated with `#[sys_cast(...)]` via the compile-time assertions the macro emits.
#[allow(unused)]
pub fn core_cast<T: SysCast>(value: T) -> T::Sys {
    assert!(
        T::validate_core(&value),
        "core_cast: invalid core value for {}",
        std::any::type_name::<T>(),
    );

    // Safety: same size/align enforced by `sys_cast` macro const assertions.
    // Validity also checked above.
    unsafe { std::mem::transmute_copy(&value) }
}
