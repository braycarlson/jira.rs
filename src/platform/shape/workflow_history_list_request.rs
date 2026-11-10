#[derive(Clone, Copy, Debug)]
pub struct WorkflowHistoryListRequest;

impl WorkflowHistoryListRequest {
    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
}
