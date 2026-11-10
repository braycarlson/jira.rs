#[derive(Clone, Copy, Debug)]
pub struct Object;

impl Object {
    pub const ATTRIBUTES: crate::Field<Self, crate::List<crate::Object<super::ObjectAttribute>>> =
        crate::Field::new("attributes");

    pub const AVATAR: crate::Field<Self, crate::Object<super::Avatar>> =
        crate::Field::new("avatar");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");

    pub const EXTENDED_INFORMATION: crate::Field<Self, crate::Any> =
        crate::Field::new("extendedInfo");

    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const HAS_AVATAR: crate::Field<Self, crate::Boolean> = crate::Field::new("hasAvatar");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");

    pub const LINKS: crate::Field<Self, crate::Object<super::ObjectLinks>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OBJECT_KEY: crate::Field<Self, crate::Text> = crate::Field::new("objectKey");

    pub const OBJECT_TYPE: crate::Field<Self, crate::Object<super::ObjectType>> =
        crate::Field::new("objectType");

    pub const TIMESTAMP: crate::Field<Self, crate::Number> = crate::Field::new("timestamp");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");
    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
