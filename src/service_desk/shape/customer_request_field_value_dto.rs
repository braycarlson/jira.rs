#[derive(Clone, Copy, Debug)]
pub struct CustomerRequestFieldValueDTO;

impl CustomerRequestFieldValueDTO {
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const RENDERED_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("renderedValue");
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");
}
