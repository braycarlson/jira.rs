#[derive(Clone, Copy, Debug)]
pub struct WorkflowPreviewResponse;

impl WorkflowPreviewResponse {
    pub const STATUSES: crate::Field<
        Self,
        crate::List<crate::Object<super::JiraWorkflowPreviewStatus>>,
    > = crate::Field::new("statuses");

    pub const WORKFLOWS: crate::Field<Self, crate::List<crate::Object<super::WorkflowPreview>>> =
        crate::Field::new("workflows");
}
