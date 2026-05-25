use rsmlui_macros::sys_cast;
pub use rsmlui_sys::Rml_Log_Type;

use crate::IntoSys;

#[sys_cast(enum(from = Rml_Log_Type, repr = i32))]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LogLevel {
    #[sys(LT_ALWAYS)]
    Always = 0,
    #[sys(LT_ERROR)]
    Error = 1,
    #[sys(LT_ASSERT)]
    Assert = 2,
    #[sys(LT_WARNING)]
    Warning = 3,
    #[sys(LT_INFO)]
    Info = 4,
    #[sys(LT_DEBUG)]
    Debug = 5,
}

pub fn message(level: LogLevel, message: String) {
    rsmlui_sys::core::log_message(level.into_sys(), message);
}

#[macro_export]
#[doc(hidden)]
macro_rules! _debug {
    ($message:literal $(, $($args:tt)*)?) => {
        $crate::core::log::message($crate::core::log::LogLevel::Debug, format!($message $(, $($args)*)?));
    };
}
pub use _debug as debug;

#[macro_export]
#[doc(hidden)]
macro_rules! _info {
    ($message:literal $(, $($args:tt)*)?) => {
        $crate::core::log::message($crate::core::log::LogLevel::Info, format!($message $(, $($args)*)?));
    };
}
pub use _info as info;

#[macro_export]
#[doc(hidden)]
macro_rules! _warning {
    ($message:literal $(, $($args:tt)*)?) => {
        $crate::core::log::message($crate::core::log::LogLevel::Warning, format!($message $(, $($args)*)?));
    };
}
pub use _warning as warning;

#[macro_export]
#[doc(hidden)]
macro_rules! _error {
    ($message:literal $(, $($args:tt)*)?) => {
        $crate::core::log::message($crate::core::log::LogLevel::Error, format!($message $(, $($args)*)?));
    };
}
pub use _error as error;

#[macro_export]
#[doc(hidden)]
macro_rules! _always {
    ($message:literal $(, $($args:tt)*)?) => {
        $crate::core::log::message($crate::core::log::LogLevel::Always, format!($message $(, $($args)*)?));
    };
}
pub use _always as always;

#[macro_export]
#[doc(hidden)]
macro_rules! _assert {
    () => {
        $crate::core::log::message($crate::core::log::LogLevel::Assert, format!("assert triggered"));
    };
    ($expr:expr) => {{
        if !{$expr} {
            $crate::core::log::message($crate::core::log::LogLevel::Assert, format!("assert triggered"));
        }
    }};
    ($expr:expr, $message:literal $(, $($args:tt)*)?) => {{
        if !{$expr} {
            $crate::core::log::message($crate::core::log::LogLevel::Assert, format!($message $(, $($args)*)?));
        }
    }};
}
pub use _assert as assert;
