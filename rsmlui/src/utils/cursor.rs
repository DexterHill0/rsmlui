#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Cursor {}

impl<'a> Into<&'a str> for Cursor {
    fn into(self) -> &'a str {
        todo!()
    }
}

impl Into<Cursor> for &str {
    fn into(self) -> Cursor {
        todo!()
    }
}
