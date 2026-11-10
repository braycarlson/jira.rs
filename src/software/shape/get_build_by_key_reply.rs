#[derive(Clone, Copy, Debug)]
pub struct GetBuildByKeyReply;

impl GetBuildByKeyReply {
    pub const ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueIDOrKeysAssociation>>,
    > = crate::Field::new("associations");

    pub const BUILD_NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("buildNumber");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");

    pub const ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueKeys");

    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");
    pub const PIPELINE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pipelineId");

    pub const REFERENCES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetBuildByKeyReplyReferences>>,
    > = crate::Field::new("references");

    pub const SCHEMA_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("schemaVersion");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");

    pub const TEST_INFORMATION: crate::Field<
        Self,
        crate::Object<super::GetBuildByKeyReplyTestInfo>,
    > = crate::Field::new("testInfo");

    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");

    pub const UPDATE_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceNumber");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
