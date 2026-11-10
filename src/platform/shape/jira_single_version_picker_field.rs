#[derive(Clone, Copy, Debug)]
pub struct JiraSingleVersionPickerField;

impl JiraSingleVersionPickerField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");

    pub const VERSION: crate::Field<Self, crate::Object<super::JiraVersionField>> =
        crate::Field::new("version");
}
