#[derive(Clone, Copy, Debug)]
pub struct WorkflowCopyRequest;

impl WorkflowCopyRequest {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
    pub const WORKFLOW_NAME: crate::Field<Self, crate::Text> = crate::Field::new("workflowName");
}
