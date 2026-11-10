#[derive(Clone, Copy, Debug)]
pub struct IssueUpdateMetadataFieldsSchema;

impl IssueUpdateMetadataFieldsSchema {
    pub const CONFIGURATION: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("configuration");

    pub const CUSTOM: crate::Field<Self, crate::Text> = crate::Field::new("custom");
    pub const CUSTOM_ID: crate::Field<Self, crate::Integer> = crate::Field::new("customId");
    pub const ITEMS: crate::Field<Self, crate::Text> = crate::Field::new("items");
    pub const SYSTEM: crate::Field<Self, crate::Text> = crate::Field::new("system");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
