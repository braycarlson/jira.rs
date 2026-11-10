#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeProjectAssociation;

impl WorkflowSchemeProjectAssociation {
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");

    pub const WORKFLOW_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("workflowSchemeId");
}
