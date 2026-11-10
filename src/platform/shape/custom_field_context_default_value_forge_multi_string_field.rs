#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueForgeMultiStringField;

impl CustomFieldContextDefaultValueForgeMultiStringField {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const VALUES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("values");
}
