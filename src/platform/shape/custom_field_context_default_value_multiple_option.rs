#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueMultipleOption;

impl CustomFieldContextDefaultValueMultipleOption {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");

    pub const OPTION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("optionIds");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
