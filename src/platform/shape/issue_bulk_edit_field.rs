#[derive(Clone, Copy, Debug)]
pub struct IssueBulkEditField;

impl IssueBulkEditField {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const FIELD_OPTIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("fieldOptions");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("isRequired");

    pub const MULTI_SELECT_FIELD_OPTIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("multiSelectFieldOptions");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const SEARCH_URL: crate::Field<Self, crate::Text> = crate::Field::new("searchUrl");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const UNAVAILABLE_MESSAGE: crate::Field<Self, crate::Text> =
        crate::Field::new("unavailableMessage");
}
