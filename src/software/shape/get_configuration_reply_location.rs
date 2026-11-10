#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReplyLocation;

impl GetConfigurationReplyLocation {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROJECT_KEY_OR_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("projectKeyOrId");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
