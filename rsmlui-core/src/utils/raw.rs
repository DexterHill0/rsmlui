pub(crate) trait Raw {
    type Target;

    fn raw(&self) -> *const Self::Target;
}

pub(crate) trait RawMut: Raw {
    fn raw_mut(&mut self) -> *mut Self::Target;
}

pub(crate) type Ptr<R> = *const <R as Raw>::Target;
pub(crate) type PtrMut<R> = *mut <R as Raw>::Target;
