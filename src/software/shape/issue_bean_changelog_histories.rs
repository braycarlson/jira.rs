#[derive(Clone, Copy, Debug)]
pub struct IssueBeanChangelogHistories;

impl IssueBeanChangelogHistories {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::IssueBeanChangelogHistoriesAuthor>> =
        crate::Field::new("author");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");

    pub const HISTORY_METADATA: crate::Field<
        Self,
        crate::Object<super::IssueBeanChangelogHistoriesHistoryMetadata>,
    > = crate::Field::new("historyMetadata");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ITEMS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueBeanChangelogHistoriesItems>>,
    > = crate::Field::new("items");
}
