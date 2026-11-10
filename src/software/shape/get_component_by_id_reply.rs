#[derive(Clone, Copy, Debug)]
pub struct GetComponentByIDReply;

impl GetComponentByIDReply {
    pub const AVATAR_URL: crate::Field<Self, crate::Text> = crate::Field::new("avatarUrl");
    pub const COMPONENT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("componentType");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PROVIDER_NAME: crate::Field<Self, crate::Text> = crate::Field::new("providerName");
    pub const SCHEMA_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("schemaVersion");
    pub const TIER: crate::Field<Self, crate::Text> = crate::Field::new("tier");
    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");

    pub const UPDATE_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceNumber");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
