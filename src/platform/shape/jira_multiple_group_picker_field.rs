#[derive(Clone, Copy, Debug)]
pub struct JiraMultipleGroupPickerField;

impl JiraMultipleGroupPickerField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const GROUPS: crate::Field<Self, crate::List<crate::Object<super::JiraGroupInput>>> =
        crate::Field::new("groups");
}
