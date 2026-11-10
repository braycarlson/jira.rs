#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueForgeDateTimeField;

impl CustomFieldContextDefaultValueForgeDateTimeField {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const DATE_TIME: crate::Field<Self, crate::Text> = crate::Field::new("dateTime");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USE_CURRENT: crate::Field<Self, crate::Boolean> = crate::Field::new("useCurrent");
}
