#[derive(Clone, Copy, Debug)]
pub struct JiraNumberField;

impl JiraNumberField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const VALUE: crate::Field<Self, crate::Number> = crate::Field::new("value");
}
