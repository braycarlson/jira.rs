#[derive(Clone, Copy, Debug)]
pub struct AttachmentArchiveImpl;

impl AttachmentArchiveImpl {
    pub const ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::AttachmentArchiveEntry>>,
    > = crate::Field::new("entries");

    pub const ENTRY_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalEntryCount");
}
