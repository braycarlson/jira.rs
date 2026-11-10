#[derive(Clone, Copy, Debug)]
pub struct AttachmentArchiveMetadataReadable;

impl AttachmentArchiveMetadataReadable {
    pub const ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::AttachmentArchiveItemReadable>>,
    > = crate::Field::new("entries");

    pub const ENTRY_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalEntryCount");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");
    pub const MEDIA_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("mediaType");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
