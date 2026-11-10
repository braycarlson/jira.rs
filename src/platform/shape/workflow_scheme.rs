#[derive(Clone, Copy, Debug)]
pub struct WorkflowScheme;

impl WorkflowScheme {
    pub const DEFAULT_WORKFLOW: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultWorkflow");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DRAFT: crate::Field<Self, crate::Boolean> = crate::Field::new("draft");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const ISSUE_TYPES: crate::Field<Self, crate::Map<crate::Object<super::IssueTypeDetails>>> =
        crate::Field::new("issueTypes");

    pub const ISSUE_TYPE_MAPPINGS: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("issueTypeMappings");

    pub const MODIFIED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastModified");

    pub const MODIFIED_USER_LAST: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("lastModifiedUser");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ORIGINAL_DEFAULT_WORKFLOW: crate::Field<Self, crate::Text> =
        crate::Field::new("originalDefaultWorkflow");

    pub const ORIGINAL_ISSUE_TYPE_MAPPINGS: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("originalIssueTypeMappings");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const UPDATE_DRAFT_IF_NEEDED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("updateDraftIfNeeded");
}
