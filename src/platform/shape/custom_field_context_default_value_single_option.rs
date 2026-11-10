#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueSingleOption;

impl CustomFieldContextDefaultValueSingleOption {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const OPTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("optionId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
