pub trait IntoSys<T> {
    fn into_sys(self) -> T;
}

pub trait FromSys<T>: Sized {
    fn from_sys(value: T) -> Self;
}

impl<T, U> IntoSys<U> for T
where
    U: FromSys<T>,
{
    fn into_sys(self) -> U {
        U::from_sys(self)
    }
}
