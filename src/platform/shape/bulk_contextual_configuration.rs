#[derive(Clone, Copy, Debug)]
pub struct BulkContextualConfiguration;

impl BulkContextualConfiguration {
    pub const CONFIGURATION: crate::Field<Self, crate::Any> = crate::Field::new("configuration");
    pub const CUSTOM_FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("customFieldId");

    pub const FIELD_CONTEXT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("fieldContextId");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SCHEMA: crate::Field<Self, crate::Any> = crate::Field::new("schema");
}
