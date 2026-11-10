#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeUpdateRequest;

impl WorkflowSchemeUpdateRequest {
    pub const DEFAULT_WORKFLOW_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultWorkflowId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const STATUS_MAPPINGS_BY_ISSUE_TYPE_OVERRIDE: crate::Field<
        Self,
        crate::List<crate::Object<super::MappingsByIssueTypeOverride>>,
    > = crate::Field::new("statusMappingsByIssueTypeOverride");

    pub const STATUS_MAPPINGS_BY_WORKFLOWS: crate::Field<
        Self,
        crate::List<crate::Object<super::MappingsByWorkflow>>,
    > = crate::Field::new("statusMappingsByWorkflows");

    pub const VERSION: crate::Field<Self, crate::Object<super::DocumentVersion>> =
        crate::Field::new("version");

    pub const WORKFLOWS_FOR_ISSUE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowSchemeAssociation>>,
    > = crate::Field::new("workflowsForIssueTypes");
}
