#[derive(Clone, Copy, Debug)]
pub struct OrderOfIssueTypes;

impl OrderOfIssueTypes {
    pub const AFTER: crate::Field<Self, crate::Text> = crate::Field::new("after");

    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const POSITION: crate::Field<Self, crate::Text> = crate::Field::new("position");
}
