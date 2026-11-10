#[derive(Clone, Copy, Debug)]
pub struct JiraSingleGroupPickerField;

impl JiraSingleGroupPickerField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const GROUP: crate::Field<Self, crate::Object<super::JiraGroupInput>> =
        crate::Field::new("group");
}
