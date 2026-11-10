#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeAssociations;

impl WorkflowSchemeAssociations {
    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");

    pub const WORKFLOW_SCHEME: crate::Field<Self, crate::Object<super::WorkflowScheme>> =
        crate::Field::new("workflowScheme");
}
