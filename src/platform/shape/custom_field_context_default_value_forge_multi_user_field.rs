#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueForgeMultiUserField;

impl CustomFieldContextDefaultValueForgeMultiUserField {
    pub const ACCOUNT_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("accountIds");

    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
