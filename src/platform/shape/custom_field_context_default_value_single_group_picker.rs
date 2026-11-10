#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueSingleGroupPicker;

impl CustomFieldContextDefaultValueSingleGroupPicker {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const GROUP_ID: crate::Field<Self, crate::Text> = crate::Field::new("groupId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
