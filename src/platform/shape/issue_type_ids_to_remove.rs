#[derive(Clone, Copy, Debug)]
pub struct IssueTypeIDsToRemove;

impl IssueTypeIDsToRemove {
    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");
}
