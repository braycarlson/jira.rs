#[derive(Clone, Copy, Debug)]
pub struct AttachmentArchiveEntry;

impl AttachmentArchiveEntry {
    pub const ABBREVIATED_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("abbreviatedName");

    pub const ENTRY_INDEX: crate::Field<Self, crate::Integer> = crate::Field::new("entryIndex");
    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
}
