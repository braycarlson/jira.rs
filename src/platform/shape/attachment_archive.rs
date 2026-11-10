#[derive(Clone, Copy, Debug)]
pub struct AttachmentArchive;

impl AttachmentArchive {
    pub const ENTRIES: crate::Field<
        Self,
        crate::List<crate::Object<super::AttachmentArchiveEntry>>,
    > = crate::Field::new("entries");

    pub const ENTRY_COUNT_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalEntryCount");

    pub const MORE_AVAILABLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("moreAvailable");

    pub const NUMBER_OF_ENTRIES_AVAILABLE_TOTAL: crate::Field<Self, crate::Integer> =
        crate::Field::new("totalNumberOfEntriesAvailable");
}
