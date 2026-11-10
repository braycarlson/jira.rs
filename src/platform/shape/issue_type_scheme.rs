#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScheme;

impl IssueTypeScheme {
    pub const DEFAULT_ISSUE_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultIssueTypeId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const ISSUE_TYPES: crate::Field<Self, crate::Any> = crate::Field::new("issueTypes");
    pub const IS_DEFAULT: crate::Field<Self, crate::Boolean> = crate::Field::new("isDefault");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PROJECTS: crate::Field<Self, crate::Any> = crate::Field::new("projects");
}
