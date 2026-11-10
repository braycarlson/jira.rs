#[derive(Clone, Copy, Debug)]
pub struct ProjectAndIssueTypePair;

impl ProjectAndIssueTypePair {
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
}
