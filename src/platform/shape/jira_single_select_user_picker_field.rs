#[derive(Clone, Copy, Debug)]
pub struct JiraSingleSelectUserPickerField;

impl JiraSingleSelectUserPickerField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const USER: crate::Field<Self, crate::Object<super::JiraUserField>> =
        crate::Field::new("user");
}
