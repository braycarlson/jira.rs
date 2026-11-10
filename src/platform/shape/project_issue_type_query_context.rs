#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueTypeQueryContext;

impl ProjectIssueTypeQueryContext {
    pub const ISSUE_TYPES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypes");

    pub const PROJECT: crate::Field<Self, crate::Text> = crate::Field::new("project");
}
