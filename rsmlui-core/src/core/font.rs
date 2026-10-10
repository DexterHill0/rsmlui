#[non_exhaustive]
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub enum FontFaceSource<'a> {
    Path(&'a str),
    Memory(&'a [u8]),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Builder)]
#[builder(const)]
pub struct FontFaceOptions {
    #[builder(default = false)]
    pub(crate) fallback_face: bool,
    #[builder(default = FontStyle::Normal)]
    pub(crate) style: FontStyle,
    #[builder(default = FontWeight::Auto)]
    pub(crate) weight: FontWeight,
    #[builder(default = 0)]
    pub(crate) face_index: i32,
}

impl Default for FontFaceOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}
