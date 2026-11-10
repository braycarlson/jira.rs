#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueForgeNumberField;

impl CustomFieldContextDefaultValueForgeNumberField {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const NUMBER: crate::Field<Self, crate::Number> = crate::Field::new("number");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
