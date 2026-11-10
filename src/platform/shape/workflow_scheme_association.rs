#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeAssociation;

impl WorkflowSchemeAssociation {
    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const WORKFLOW_ID: crate::Field<Self, crate::Text> = crate::Field::new("workflowId");
}
