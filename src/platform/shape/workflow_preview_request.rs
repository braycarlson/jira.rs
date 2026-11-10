#[derive(Clone, Copy, Debug)]
pub struct WorkflowPreviewRequest;

impl WorkflowPreviewRequest {
    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");

    pub const WORKFLOW_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workflowIds");

    pub const WORKFLOW_NAMES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workflowNames");
}
