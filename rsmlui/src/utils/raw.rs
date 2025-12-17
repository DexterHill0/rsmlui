pub(crate) trait Raw {
    type Ptr;

    fn raw(&self) -> Self::Ptr;
}

pub(crate) type Ptr<R> = <R as Raw>::Ptr;
