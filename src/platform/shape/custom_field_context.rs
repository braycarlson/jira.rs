#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContext;

impl CustomFieldContext {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const IS_ANY_ISSUE_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isAnyIssueType");

    pub const IS_GLOBAL_CONTEXT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isGlobalContext");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
