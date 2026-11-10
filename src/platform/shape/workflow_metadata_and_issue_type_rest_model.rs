#[derive(Clone, Copy, Debug)]
pub struct WorkflowMetadataAndIssueTypeRestModel;

impl WorkflowMetadataAndIssueTypeRestModel {
    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const WORKFLOW: crate::Field<Self, crate::Object<super::WorkflowMetadataRestModel>> =
        crate::Field::new("workflow");
}
