#[derive(Clone, Copy, Debug)]
pub struct AttachmentArchiveItemReadable;

impl AttachmentArchiveItemReadable {
    pub const INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("index");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");
    pub const PATH: crate::Field<Self, crate::Text> = crate::Field::new("path");
    pub const SIZE: crate::Field<Self, crate::Text> = crate::Field::new("size");
}
