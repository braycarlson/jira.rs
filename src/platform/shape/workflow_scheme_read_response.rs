#[derive(Clone, Copy, Debug)]
pub struct WorkflowSchemeReadResponse;

impl WorkflowSchemeReadResponse {
    pub const DEFAULT_WORKFLOW: crate::Field<
        Self,
        crate::Object<super::WorkflowMetadataRestModel>,
    > = crate::Field::new("defaultWorkflow");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SCOPE: crate::Field<Self, crate::Object<super::WorkflowScope>> =
        crate::Field::new("scope");

    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");

    pub const VERSION: crate::Field<Self, crate::Object<super::DocumentVersion>> =
        crate::Field::new("version");

    pub const WORKFLOWS_FOR_ISSUE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::WorkflowMetadataAndIssueTypeRestModel>>,
    > = crate::Field::new("workflowsForIssueTypes");
}
