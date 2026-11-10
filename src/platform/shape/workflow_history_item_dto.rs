#[derive(Clone, Copy, Debug)]
pub struct WorkflowHistoryItemDTO;

impl WorkflowHistoryItemDTO {
    pub const IS_INTERMEDIATE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isIntermediate");

    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");

    pub const WORKFLOW_VERSION: crate::Field<Self, crate::Integer> =
        crate::Field::new("workflowVersion");

    pub const WRITTEN_AT: crate::Field<Self, crate::Text> = crate::Field::new("writtenAt");
}
