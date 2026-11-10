#[derive(Clone, Copy, Debug)]
pub struct IssueTypeIDs;

impl IssueTypeIDs {
    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");
}
