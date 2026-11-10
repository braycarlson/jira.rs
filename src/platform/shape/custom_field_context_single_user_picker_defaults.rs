#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextSingleUserPickerDefaults;

impl CustomFieldContextSingleUserPickerDefaults {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const USER_FILTER: crate::Field<Self, crate::Object<super::UserFilter>> =
        crate::Field::new("userFilter");
}
