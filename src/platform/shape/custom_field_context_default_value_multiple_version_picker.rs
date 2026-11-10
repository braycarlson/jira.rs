#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueMultipleVersionPicker;

impl CustomFieldContextDefaultValueMultipleVersionPicker {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("versionIds");

    pub const VERSION_ORDER: crate::Field<Self, crate::Text> = crate::Field::new("versionOrder");
}
