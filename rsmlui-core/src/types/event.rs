#[must_use]
pub enum EventFlow {
    Propagated,
    Stopped,
}

impl From<bool> for EventFlow {
    fn from(value: bool) -> Self {
        if value {
            EventFlow::Propagated
        } else {
            EventFlow::Stopped
        }
    }
}
