#[derive(Clone, Copy, Debug)]
pub struct ObjectSchemaIn;

impl ObjectSchemaIn {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_SCHEMA_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaKey");
}
