#[derive(Clone, Copy, Debug)]
pub struct CreateCustomFieldContext;

impl CreateCustomFieldContext {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_TYPE_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueTypeIds");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROJECT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("projectIds");
}
