#[macro_export]
macro_rules! export_interfaces {
    (
        default: $default:ident;

        $(
            #[$($attrs:meta)*]
            $module:ident::$interface:ident
        )*
    ) => {
        $(
            #[$($attrs)*]
            pub mod $module;
            #[$($attrs)*]
            pub type $default = $module::$interface;
        )*
    };
}

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
