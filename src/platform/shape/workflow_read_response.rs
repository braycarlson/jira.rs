#[derive(Clone, Copy, Debug)]
pub struct WorkflowReadResponse;

impl WorkflowReadResponse {
    pub const STATUSES: crate::Field<Self, crate::List<crate::Object<super::JiraWorkflowStatus>>> =
        crate::Field::new("statuses");

    pub const WORKFLOWS: crate::Field<Self, crate::List<crate::Object<super::JiraWorkflow>>> =
        crate::Field::new("workflows");
}
