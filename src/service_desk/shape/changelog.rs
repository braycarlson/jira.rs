#[derive(Clone, Copy, Debug)]
pub struct Changelog;

impl Changelog {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("author");

    pub const CREATED: crate::Field<Self, crate::Any> = crate::Field::new("created");

    pub const HISTORY_METADATA: crate::Field<Self, crate::Object<super::HistoryMetadata>> =
        crate::Field::new("historyMetadata");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ITEMS: crate::Field<Self, crate::List<crate::Object<super::ChangeDetails>>> =
        crate::Field::new("items");
}
