#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueSingleVersionPicker;

impl CustomFieldContextDefaultValueSingleVersionPicker {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VERSION_ID: crate::Field<Self, crate::Text> = crate::Field::new("versionId");
    pub const VERSION_ORDER: crate::Field<Self, crate::Text> = crate::Field::new("versionOrder");
}
