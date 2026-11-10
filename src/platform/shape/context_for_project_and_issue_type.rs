#[derive(Clone, Copy, Debug)]
pub struct ContextForProjectAndIssueType;

impl ContextForProjectAndIssueType {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
