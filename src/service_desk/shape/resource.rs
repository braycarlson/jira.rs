#[derive(Clone, Copy, Debug)]
pub struct Resource;

impl Resource {
    pub const CONTENT_AS_BYTE_ARRAY: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("contentAsByteArray");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const FILE: crate::Field<Self, crate::Text> = crate::Field::new("file");
    pub const FILENAME: crate::Field<Self, crate::Text> = crate::Field::new("filename");
    pub const INPUT_STREAM: crate::Field<Self, crate::Any> = crate::Field::new("inputStream");
    pub const OPEN: crate::Field<Self, crate::Boolean> = crate::Field::new("open");
    pub const READABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("readable");
    pub const URI: crate::Field<Self, crate::Text> = crate::Field::new("uri");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
