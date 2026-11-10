#[derive(Clone, Copy, Debug)]
pub struct WorkflowProjectUsageDTO;

impl WorkflowProjectUsageDTO {
    pub const PROJECTS: crate::Field<Self, crate::Object<super::ProjectUsagePage>> =
        crate::Field::new("projects");

    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
}
