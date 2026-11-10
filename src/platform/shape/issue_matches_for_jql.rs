#[derive(Clone, Copy, Debug)]
pub struct IssueMatchesForJQL;

impl IssueMatchesForJQL {
    pub const ERRORS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("errors");

    pub const MATCHED_ISSUES: crate::Field<Self, crate::List<crate::Integer>> =
        crate::Field::new("matchedIssues");
}
