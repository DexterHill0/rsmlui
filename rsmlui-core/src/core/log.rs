use rsmlui_macros::sys_cast;
pub use rsmlui_sys::Rml_Log_Type;

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
    #[sys(LT_MAX)]
    Max = 6,
}
