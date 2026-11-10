#[derive(Clone, Copy, Debug)]
pub struct SchemaUsageInfo;

impl SchemaUsageInfo {
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const OBJECT_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("objectCount");

    pub const SCHEMA_CREATED_AT: crate::Field<Self, crate::Text> =
        crate::Field::new("schemaCreatedAt");

    pub const SCHEMA_ID: crate::Field<Self, crate::Integer> = crate::Field::new("schemaId");
    pub const SCHEMA_NAME: crate::Field<Self, crate::Text> = crate::Field::new("schemaName");
}
