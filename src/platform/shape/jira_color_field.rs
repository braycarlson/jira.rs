#[derive(Clone, Copy, Debug)]
pub struct JiraColorField;

impl JiraColorField {
    pub const COLOR: crate::Field<Self, crate::Object<super::JiraColorInput>> =
        crate::Field::new("color");

    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
}
