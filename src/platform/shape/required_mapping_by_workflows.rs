#[derive(Clone, Copy, Debug)]
pub struct RequiredMappingByWorkflows;

impl RequiredMappingByWorkflows {
    pub const SOURCE_WORKFLOW_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("sourceWorkflowId");

    pub const STATUS_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("statusIds");

    pub const TARGET_WORKFLOW_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("targetWorkflowId");
}
