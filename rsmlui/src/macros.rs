#[doc(hidden)]
#[macro_export]
macro_rules! _backend {
    [$system:path, $render:path, $file:path, $window:path $(,)?] => {
        ::rsmlui::backend::Backend<
            <$system as ::rsmlui::_private::HasOwnedInterface<{rsmlui::_private::SYSTEM_INTERFACE_N}>>::Owned,
            <$render as ::rsmlui::_private::HasOwnedInterface<{rsmlui::_private::RENDER_INTERFACE_N}>>::Owned,
            <$file as ::rsmlui::_private::HasOwnedInterface<{rsmlui::_private::FILE_INTERFACE_N}>>::Owned,
            $window,
        >
    };
}

pub use _backend as backend;
