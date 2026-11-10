#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeProjectUsageDTO;

impl WorkflowSchemeProjectUsageDTO {
    pub const PROJECTS: crate::Field<Self, crate::Object<super::ProjectUsagePage>> =
        crate::Field::new("projects");

    pub const WORKFLOW_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("workflowSchemeId");
}
