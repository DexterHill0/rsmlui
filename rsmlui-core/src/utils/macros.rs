// makes a type !Send and !Sync
#[doc(hidden)]
#[macro_export]
macro_rules! _not_send_sync {
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

pub use _not_send_sync as not_send_sync;
