#[derive(Clone, Copy, Debug)]
pub struct AttachmentMetadata;

impl AttachmentMetadata {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("author");
    pub const CONTENT: crate::Field<Self, crate::Text> = crate::Field::new("content");
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const FILENAME: crate::Field<Self, crate::Text> = crate::Field::new("filename");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const MIME_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mimeType");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const THUMBNAIL: crate::Field<Self, crate::Text> = crate::Field::new("thumbnail");
}
