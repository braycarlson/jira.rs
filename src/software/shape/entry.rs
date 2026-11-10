#[derive(Clone, Copy, Debug)]
pub struct Entry;

impl Entry {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("errors");
    pub const ISSUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("issueId");
    pub const ISSUE_KEY: crate::Field<Self, crate::Text> = crate::Field::new("issueKey");
    pub const STATUS: crate::Field<Self, crate::Integer> = crate::Field::new("status");
}
