pub mod conversions;
pub mod cursor;
pub mod input;
pub mod raw;

// makes a type !Send and !Sync
#[macro_export]
macro_rules! not_send_sync {
    ($($name:ident),* $(,)?) => {
        $(
            impl !Send for $name {}
            impl !Sync for $name {}
        )*
    };

    (
        $(
            [$($impl_generics:tt)*] $name:ident [$($ty_generics:tt)*]
        )*
    ) => {
        $(
            impl<$($impl_generics)*> !Send for $name<$($ty_generics)*> {}
            impl<$($impl_generics)*> !Sync for $name<$($ty_generics)*> {}
        )*
    };
}
