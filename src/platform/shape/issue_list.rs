#[derive(Clone, Copy, Debug)]
pub struct IssueList;

impl IssueList {
    pub const ISSUE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueIds");
}
