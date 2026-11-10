#[derive(Clone, Copy, Debug)]
pub struct ObjectType;

impl ObjectType {
    pub const ABSTRACT_OBJECT_TYPE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("abstractObjectType");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");

    pub const DELETION_SWEEP_TASK_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("deletionSweepTaskId");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ICON: crate::Field<Self, crate::Object<super::Icon>> = crate::Field::new("icon");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const INHERITED: crate::Field<Self, crate::Boolean> = crate::Field::new("inherited");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OBJECT_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("objectCount");

    pub const OBJECT_SCHEMA_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaId");

    pub const PARENT_OBJECT_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("parentObjectTypeId");

    pub const PARENT_OBJECT_TYPE_INHERITED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("parentObjectTypeInherited");

    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");

    pub const SCHEMA_DISPLAY_LABEL: crate::Field<Self, crate::Text> =
        crate::Field::new("schemaDisplayLabel");

    pub const SCHEMA_LABEL: crate::Field<Self, crate::Text> = crate::Field::new("schemaLabel");
    pub const TYPE: crate::Field<Self, crate::Integer> = crate::Field::new("type");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
