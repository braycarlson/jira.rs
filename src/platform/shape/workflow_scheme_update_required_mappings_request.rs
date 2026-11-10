#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeUpdateRequiredMappingsRequest;

impl WorkflowSchemeUpdateRequiredMappingsRequest {
    pub const DEFAULT_WORKFLOW_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultWorkflowId");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const WORKFLOWS_FOR_ISSUE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowSchemeAssociation>>,
    > = crate::Field::new("workflowsForIssueTypes");
}
