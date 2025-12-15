pub(crate) trait Raw {
    type Ptr;
}

pub(crate) type Ptr<R> = <R as Raw>::Ptr;
