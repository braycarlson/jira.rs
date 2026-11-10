#[derive(Clone, Copy, Debug)]
pub struct PageOfChangelogsHistories;

impl PageOfChangelogsHistories {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::PageOfChangelogsHistoriesAuthor>> =
        crate::Field::new("author");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");

    pub const HISTORY_METADATA: crate::Field<
        Self,
        crate::Object<super::PageOfChangelogsHistoriesHistoryMetadata>,
    > = crate::Field::new("historyMetadata");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ITEMS: crate::Field<
        Self,
        crate::List<crate::Object<super::PageOfChangelogsHistoriesItems>>,
    > = crate::Field::new("items");
}
