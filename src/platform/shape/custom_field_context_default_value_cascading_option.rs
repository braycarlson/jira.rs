#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueCascadingOption;

impl CustomFieldContextDefaultValueCascadingOption {
    pub const CASCADING_OPTION_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("cascadingOptionId");

    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const OPTION_ID: crate::Field<Self, crate::Text> = crate::Field::new("optionId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
