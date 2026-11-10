#[derive(Clone, Copy, Debug)]
pub struct StatusesPerWorkflow;

impl StatusesPerWorkflow {
    pub const STATUSES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("statuses");

    pub const STATUS_ID_INITIAL: crate::Field<Self, crate::Text> =
        crate::Field::new("initialStatusId");

    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
}
