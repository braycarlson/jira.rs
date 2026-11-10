#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueMultipleGroupPicker;

impl CustomFieldContextDefaultValueMultipleGroupPicker {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");

    pub const GROUP_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("groupIds");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
