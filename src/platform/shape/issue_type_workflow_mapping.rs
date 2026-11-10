#[derive(Clone, Copy, Debug)]
pub struct IssueTypeWorkflowMapping;

impl IssueTypeWorkflowMapping {
    pub const ISSUE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("issueType");

    pub const UPDATE_DRAFT_IF_NEEDED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("updateDraftIfNeeded");

    pub const WORKFLOW: crate::Field<Self, crate::Text> = crate::Field::new("workflow");
}
