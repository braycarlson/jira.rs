#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueLongText;

impl CustomFieldContextDefaultValueLongText {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TEXT: crate::Field<Self, crate::Text> = crate::Field::new("text");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
