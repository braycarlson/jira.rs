#[derive(Clone, Copy, Debug)]
pub struct ProjectIssueTypeMapping;

impl ProjectIssueTypeMapping {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
