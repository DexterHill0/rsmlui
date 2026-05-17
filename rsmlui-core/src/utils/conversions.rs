pub trait FromSys<T> {
    fn from_sys(value: T) -> Self;
}

pub trait IntoSys<T> {
    fn into_sys(self) -> T;
}
