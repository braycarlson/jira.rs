#[derive(Clone, Copy, Debug)]
pub struct WorkflowReadRequest;

impl WorkflowReadRequest {
    pub const PROJECT_AND_ISSUE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::ProjectAndIssueTypePair>>,
    > = crate::Field::new("projectAndIssueTypes");

    pub const WORKFLOW_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workflowIds");

    pub const WORKFLOW_NAMES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("workflowNames");
}
