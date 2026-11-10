#[derive(Clone, Copy, Debug)]
pub struct ObjectSchema;

impl ObjectSchema {
    pub const ASYNC_TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("asyncTaskId");

    pub const ATLASSIAN_TEMPLATE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("atlassianTemplateId");

    pub const CAN_MANAGE: crate::Field<Self, crate::Boolean> = crate::Field::new("canManage");
    pub const CDM_DATA: crate::Field<Self, crate::Any> = crate::Field::new("cdmData");
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");

    pub const CROSS_SCHEMA_REFERENCING_ALLOWED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("crossSchemaReferencingAllowed");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OBJECT_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("objectCount");

    pub const OBJECT_SCHEMA_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("objectSchemaKey");

    pub const OBJECT_TYPE_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("objectTypeCount");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
