#[doc(hidden)]
#[macro_export]
macro_rules! _backend {
    [$system:path, $render:path, $file:path, $window:path $(,)?] => {
        ::rsmlui::backend::Backend<
            <$system as ::rsmlui::_private::HasOwnedInterface<0>>::Owned,
            <$render as ::rsmlui::_private::HasOwnedInterface<1>>::Owned,
            <$file as ::rsmlui::_private::HasOwnedInterface<2>>::Owned,
            $window,
        >
    };
}

pub use _backend as backend;
