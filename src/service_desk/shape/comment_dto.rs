#[derive(Clone, Copy, Debug)]
pub struct CommentDTO;

impl CommentDTO {
    pub const ATTACHMENTS: crate::Field<Self, crate::Object<super::PagedDTOAttachmentDTO>> =
        crate::Field::new("attachments");

    pub const AUTHOR: crate::Field<Self, crate::Object<super::UserDTO>> =
        crate::Field::new("author");

    pub const BODY: crate::Field<Self, crate::Text> = crate::Field::new("body");

    pub const CREATED: crate::Field<Self, crate::Object<super::DateDTO>> =
        crate::Field::new("created");

    pub const EXPANDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("_expands");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const PUBLIC: crate::Field<Self, crate::Boolean> = crate::Field::new("public");

    pub const RENDERED_BODY: crate::Field<Self, crate::Object<super::RenderedValueDTO>> =
        crate::Field::new("renderedBody");
}
