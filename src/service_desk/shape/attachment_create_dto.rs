#[derive(Clone, Copy, Debug)]
pub struct AttachmentCreateDTO;

impl AttachmentCreateDTO {
    pub const ADDITIONAL_COMMENT: crate::Field<Self, crate::Object<super::AdditionalCommentDTO>> =
        crate::Field::new("additionalComment");

    pub const PUBLIC: crate::Field<Self, crate::Boolean> = crate::Field::new("public");

    pub const TEMPORARY_ATTACHMENT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("temporaryAttachmentIds");
}
