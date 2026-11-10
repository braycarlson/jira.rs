#[derive(Clone, Copy, Debug)]
pub struct Context;

impl Context {
    pub const ASSOCIATED_ISSUE_TYPES: crate::Field<Self, crate::Any> =
        crate::Field::new("associatedIssueTypes");

    pub const ASSOCIATED_PROJECTS: crate::Field<Self, crate::Any> =
        crate::Field::new("associatedProjects");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const IS_ALL_ISSUE_TYPES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAllIssueTypes");

    pub const IS_ALL_PROJECTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAllProjects");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SCOPE: crate::Field<Self, crate::Object<super::Scope>> = crate::Field::new("scope");
}
