#[derive(Clone, Copy, Debug)]
pub struct Status;

impl Status {
    pub const CATEGORY: crate::Field<Self, crate::Integer> = crate::Field::new("category");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");

    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
