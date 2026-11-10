#[derive(Clone, Copy, Debug)]
pub struct LinkableAttachmentLinkDTO;

impl LinkableAttachmentLinkDTO {
    pub const LINKS: crate::Field<Self, crate::Object<super::AttachmentLinkDTO>> =
        crate::Field::new("_links");
}
