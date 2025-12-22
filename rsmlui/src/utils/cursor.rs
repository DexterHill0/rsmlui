#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[non_exhaustive]
pub enum Cursor {
    #[default]
    Arrow,
    Move,
    Pointer,
    Resize,
    Cross,
    Text,
    Unavailable,
    Scroll(String),
    Unknown(String),
}

impl Into<String> for Cursor {
    fn into(self) -> String {
        match self {
            Cursor::Arrow => "arrow".to_owned(),
            Cursor::Move => "move".to_owned(),
            Cursor::Pointer => "pointer".to_owned(),
            Cursor::Resize => "resize".to_owned(),
            Cursor::Cross => "cross".to_owned(),
            Cursor::Text => "text".to_owned(),
            Cursor::Unavailable => "unavailable".to_owned(),
            Cursor::Scroll(kind) => format!("rmlui-scroll-{kind}"),
            Cursor::Unknown(kind) => kind,
        }
    }
}

impl Into<Cursor> for &str {
    fn into(self) -> Cursor {
        match self {
            "" | "arrow" => Cursor::Arrow,
            "move" => Cursor::Move,
            "pointer" => Cursor::Pointer,
            "resize" => Cursor::Resize,
            "cross" => Cursor::Cross,
            "text" => Cursor::Text,
            "unavailable" => Cursor::Unavailable,
            scroll if scroll.starts_with("rmlui-scroll") => {
                Cursor::Scroll(scroll.replace("rmlui-scroll", ""))
            },
            _ => Cursor::Unknown(self.into()),
        }
    }
}
