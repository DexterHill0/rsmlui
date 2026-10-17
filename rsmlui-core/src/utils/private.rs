/// Uconstructable type used to prevent callers from invoking trait methods that
/// should be private but must be public to be used in other workspace crates.
pub struct Uncallable {
    _private: (),
}

impl Uncallable {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}
