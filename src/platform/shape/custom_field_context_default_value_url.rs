#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueURL;

impl CustomFieldContextDefaultValueURL {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
