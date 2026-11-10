#[derive(Clone, Copy, Debug)]
pub struct ReferenceType;

impl ReferenceType {
    pub const CDM_DATA: crate::Field<Self, crate::Object<super::ReferenceTypeCdmData>> =
        crate::Field::new("cdmData");

    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");

    pub const REMOVABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("removable");
    pub const URL16: crate::Field<Self, crate::Text> = crate::Field::new("url16");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
