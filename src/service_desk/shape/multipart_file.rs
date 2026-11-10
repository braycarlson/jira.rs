#[derive(Clone, Copy, Debug)]
pub struct MultipartFile;

impl MultipartFile {
    pub const BYTES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("bytes");
    pub const CONTENT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("contentType");
    pub const EMPTY: crate::Field<Self, crate::Boolean> = crate::Field::new("empty");
    pub const INPUT_STREAM: crate::Field<Self, crate::Any> = crate::Field::new("inputStream");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ORIGINAL_FILENAME: crate::Field<Self, crate::Text> =
        crate::Field::new("originalFilename");

    pub const RESOURCE: crate::Field<Self, crate::Object<super::Resource>> =
        crate::Field::new("resource");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
}
