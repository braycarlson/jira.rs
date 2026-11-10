#[derive(Clone, Copy, Debug)]
pub struct AttachmentDTO;

impl AttachmentDTO {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::UserDTO>> =
        crate::Field::new("author");

    pub const CREATED: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("created");

    pub const FILENAME: crate::Field<Self, crate::Text> = crate::Field::new("filename");

    pub const LINKS: crate::Field<Self, crate::Object<super::AttachmentLinkDTO>> =
        crate::Field::new("_links");

    pub const MIME_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mimeType");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
}
