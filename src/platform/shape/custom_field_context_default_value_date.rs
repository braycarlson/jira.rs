#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueDate;

impl CustomFieldContextDefaultValueDate {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const DATE: crate::Field<Self, crate::Text> = crate::Field::new("date");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const USE_CURRENT: crate::Field<Self, crate::Boolean> = crate::Field::new("useCurrent");
}
