#[derive(Clone, Copy, Debug)]
pub struct BulkIssueResults;

impl BulkIssueResults {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const ISSUES: crate::Field<Self, crate::List<crate::Object<super::IssueBean>>> =
        crate::Field::new("issues");

    pub const ISSUE_ERRORS: crate::Field<Self, crate::List<crate::Object<super::IssueError>>> =
        crate::Field::new("issueErrors");
}
