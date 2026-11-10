#[derive(Clone, Copy, Debug)]
pub struct WorkflowUpdateResponse;

impl WorkflowUpdateResponse {
    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::JiraWorkflowStatus>>> =
        crate::Field::new("statuses");

    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");

    pub const WORKFLOWS: crate::Field<Self, crate::List<crate::Object<super::JiraWorkflow>>> =
        crate::Field::new("workflows");
}
