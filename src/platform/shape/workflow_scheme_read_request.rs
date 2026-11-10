#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeReadRequest;

impl WorkflowSchemeReadRequest {
    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");

    pub const WORKFLOW_SCHEME_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workflowSchemeIds");
}
