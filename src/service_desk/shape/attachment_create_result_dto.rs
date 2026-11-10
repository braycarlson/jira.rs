#[derive(Clone, Copy, Debug)]
pub struct AttachmentCreateResultDTO;

impl AttachmentCreateResultDTO {
    pub const ATTACHMENTS: crate::Field<Self, crate::Object<super::PagedDTOAttachmentDTO>> =
        crate::Field::new("attachments");

    pub const COMMENT: crate::Field<Self, crate::Object<super::CommentDTO>> =
        crate::Field::new("comment");
}
