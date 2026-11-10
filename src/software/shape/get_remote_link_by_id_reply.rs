#[derive(Clone, Copy, Debug)]
pub struct GetRemoteLinkByIDReply;

impl GetRemoteLinkByIDReply {
    pub const ACTION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("actionIds");

    pub const ASSOCIATIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("associations");

    pub const ATTRIBUTE_MAP: crate::Field<Self, crate::Map<crate::Text>> =
        crate::Field::new("attributeMap");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SCHEMA_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("schemaVersion");

    pub const STATUS: crate::Field<Self, crate::Object<super::GetRemoteLinkByIDReplyStatus>> =
        crate::Field::new("status");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");

    pub const UPDATE_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceNumber");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
