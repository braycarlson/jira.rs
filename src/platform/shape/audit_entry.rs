#[derive(Clone, Copy, Debug)]
pub struct AuditEntry;

impl AuditEntry {
    pub const CALLER: crate::Field<Self, crate::Text> = crate::Field::new("caller");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const FILTER_ID: crate::Field<Self, crate::Text> = crate::Field::new("filterId");
    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");
    pub const REQUEST_URL: crate::Field<Self, crate::Text> = crate::Field::new("requestUrl");
    pub const USER: crate::Field<Self, crate::Object<super::AuditUser>> = crate::Field::new("user");
    pub const USER_AGENT: crate::Field<Self, crate::Text> = crate::Field::new("userAgent");

    pub const VALIDATION_MESSAGE: crate::Field<Self, crate::Text> =
        crate::Field::new("validationMessage");
}
