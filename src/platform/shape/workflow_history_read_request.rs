#[derive(Clone, Copy, Debug)]
pub struct WorkflowHistoryReadRequest;

impl WorkflowHistoryReadRequest {
    pub const VERSION: crate::Field<Self, crate::Integer> = crate::Field::new("version");
    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
}
