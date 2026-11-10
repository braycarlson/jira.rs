#[derive(Clone, Copy, Debug)]
pub struct ObjectTypeIn;

impl ObjectTypeIn {
    pub const ABSTRACT_OBJECT_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("abstractObjectType");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ICON_ID: crate::Field<Self, crate::Text> = crate::Field::new("iconId");
    pub const INHERITED: crate::Field<Self, crate::Boolean> = crate::Field::new("inherited");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");

    pub const PARENT_OBJECT_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("parentObjectTypeId");
}
