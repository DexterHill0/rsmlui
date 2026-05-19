use syn::spanned::Spanned;

pub(crate) trait ErrorSpan {
    fn error<T: Into<String>>(&self, message: T) -> syn::Error;
}

impl<T: Spanned> ErrorSpan for T {
    fn error<U: Into<String>>(&self, message: U) -> syn::Error {
        syn::Error::new(self.span(), message.into())
    }
}
