#[derive(Clone, Copy, Debug)]
pub struct JiraMultipleSelectUserPickerField;

impl JiraMultipleSelectUserPickerField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const USERS: crate::Field<Self, crate::List<crate::Object<super::JiraUserField>>> =
        crate::Field::new("users");
}
