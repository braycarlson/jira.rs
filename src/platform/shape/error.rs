#[derive(Clone, Copy, Debug)]
pub struct Error;

impl Error {
    pub const COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("count");

    pub const ISSUE_IDS_OR_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIdsOrKeys");

    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
}
