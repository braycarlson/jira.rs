#[derive(Clone, Copy, Debug)]
pub struct IssueTypesWorkflowMapping;

impl IssueTypesWorkflowMapping {
    pub const DEFAULT_MAPPING: crate::Field<Self, crate::Boolean> =
        crate::Field::new("defaultMapping");

    pub const ISSUE_TYPES: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypes");

    pub const UPDATE_DRAFT_IF_NEEDED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("updateDraftIfNeeded");

    pub const WORKFLOW: crate::Field<Self, crate::Text> = crate::Field::new("workflow");
}
