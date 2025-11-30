pub trait IntoPtr<Target> {
    fn into_ptr(self) -> *mut Target;
}
