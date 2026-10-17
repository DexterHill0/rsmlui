#[doc(hidden)]
#[macro_export]
macro_rules! _backend {
    [$system:path, $render:path, $file:path, $window:path $(,)?] => {
        $crate::backend::Backend<
            <$system as $crate::_private::HasOwnedInterface<$crate::_private::RmlSystemInterface>>::Owned,
            <$render as $crate::_private::HasOwnedInterface<$crate::_private::RmlRenderInterface>>::Owned,
            <$file   as $crate::_private::HasOwnedInterface<$crate::_private::RmlFileInterface>>::Owned,
            $window,
        >
    };
}

/// backend! [ ]
pub use _backend as backend;
