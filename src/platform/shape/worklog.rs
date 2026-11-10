#[derive(Clone, Copy, Debug)]
pub struct Worklog;

impl Worklog {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("author");

    pub const COMMENT: crate::Field<Self, crate::Any> = crate::Field::new("comment");
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const ISSUE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueId");

    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Object<super::EntityProperty>>> =
        crate::Field::new("properties");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const STARTED: crate::Field<Self, crate::Text> = crate::Field::new("started");
    pub const TIME_SPENT: crate::Field<Self, crate::Text> = crate::Field::new("timeSpent");

    pub const TIME_SPENT_SECONDS: crate::Field<Self, crate::Integer> =
        crate::Field::new("timeSpentSeconds");

    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");

    pub const UPDATE_AUTHOR: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("updateAuthor");

    pub const VISIBILITY: crate::Field<Self, crate::Object<super::Visibility>> =
        crate::Field::new("visibility");
}
