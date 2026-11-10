#[derive(Clone, Copy, Debug)]
pub struct FieldMetadata;

impl FieldMetadata {
    pub const ALLOWED_VALUES: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("allowedValues");

    pub const AUTO_COMPLETE_URL: crate::Field<Self, crate::Text> =
        crate::Field::new("autoCompleteUrl");

    pub const CONFIGURATION: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("configuration");

    pub const DEFAULT_VALUE: crate::Field<Self, crate::Any> = crate::Field::new("defaultValue");

    pub const HAS_DEFAULT_VALUE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasDefaultValue");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OPERATIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("operations");

    pub const REQUIRED: crate::Field<Self, crate::Boolean> = crate::Field::new("required");

    pub const SCHEMA: crate::Field<Self, crate::Object<super::JSONTypeBean>> =
        crate::Field::new("schema");
}
