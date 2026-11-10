#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeUsageDTO;

impl WorkflowSchemeUsageDTO {
    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");

    pub const WORKFLOW_SCHEMES: crate::Field<Self, crate::Object<super::WorkflowSchemeUsagePage>> =
        crate::Field::new("workflowSchemes");
}
