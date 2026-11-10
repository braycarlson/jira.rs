#[derive(Clone, Copy, Debug)]
pub struct WorkflowProjectIssueTypeUsageDTO;

impl WorkflowProjectIssueTypeUsageDTO {
    pub const ISSUE_TYPES: crate::Field<
        Self,
        crate::Object<super::WorkflowProjectIssueTypeUsagePage>,
    > = crate::Field::new("issueTypes");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
}
