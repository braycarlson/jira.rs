#[derive(Clone, Copy, Debug)]
pub struct GetFeatureFlagByIDReply;

impl GetFeatureFlagByIDReply {
    pub const ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("associations");

    pub const DETAILS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetFeatureFlagByIDReplyDetails>>,
    > = crate::Field::new("details");

    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueKeys");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const SCHEMA_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("schemaVersion");

    pub const SUMMARY: crate::Field<Self, crate::Object<super::GetFeatureFlagByIDReplySummary>> =
        crate::Field::new("summary");

    pub const UPDATE_SEQUENCE_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceId");
}
