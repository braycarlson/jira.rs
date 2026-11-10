#[derive(Clone, Copy, Debug)]
pub struct JiraURLField;

impl JiraURLField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
