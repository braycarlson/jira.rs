#[derive(Clone, Copy, Debug)]
pub struct ArticleDTO;

impl ArticleDTO {
    pub const CONTENT: crate::Field<Self, crate::Object<super::ContentDTO>> =
        crate::Field::new("content");

    pub const EXCERPT: crate::Field<Self, crate::Text> = crate::Field::new("excerpt");

    pub const SOURCE: crate::Field<Self, crate::Object<super::SourceDTO>> =
        crate::Field::new("source");

    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
