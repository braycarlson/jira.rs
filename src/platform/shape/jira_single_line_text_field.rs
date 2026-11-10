#[derive(Clone, Copy, Debug)]
pub struct JiraSingleLineTextField;

impl JiraSingleLineTextField {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const TEXT: crate::Field<Self, crate::Text> = crate::Field::new("text");
}
