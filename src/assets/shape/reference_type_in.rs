#[derive(Clone, Copy, Debug)]
pub struct ReferenceTypeIn;

impl ReferenceTypeIn {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");
}
