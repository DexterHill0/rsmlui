#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum WindowEvent {
    ExitRequested,
    RenderRequested,
}
