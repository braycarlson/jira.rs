#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueForgeObjectField;

impl CustomFieldContextDefaultValueForgeObjectField {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const OBJECT: crate::Field<Self, crate::Any> = crate::Field::new("object");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
