#[derive(Clone, Copy, Debug)]
pub struct ContextualConfiguration;

impl ContextualConfiguration {
    pub const CONFIGURATION: crate::Field<Self, crate::Any> = crate::Field::new("configuration");

    pub const FIELD_CONTEXT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("fieldContextId");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SCHEMA: crate::Field<Self, crate::Any> = crate::Field::new("schema");
}
