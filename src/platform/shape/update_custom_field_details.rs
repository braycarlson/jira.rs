#[derive(Clone, Copy, Debug)]
pub struct UpdateCustomFieldDetails;

impl UpdateCustomFieldDetails {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SEARCHER_KEY: crate::Field<Self, crate::Text> = crate::Field::new("searcherKey");
}
