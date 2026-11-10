#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentByKeyReply;

impl GetDeploymentByKeyReply {
    pub const ASSOCIATIONS: crate::Field<Self, crate::List<crate::Any>> =
        crate::Field::new("associations");

    pub const COMMANDS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetDeploymentByKeyReplyCommands>>,
    > = crate::Field::new("commands");

    pub const DEPLOYMENT_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("deploymentSequenceNumber");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const DURATION: crate::Field<Self, crate::Integer> = crate::Field::new("duration");

    pub const ENVIRONMENT: crate::Field<
        Self,
        crate::Object<super::GetDeploymentByKeyReplyEnvironment>,
    > = crate::Field::new("environment");

    pub const ISSUE_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("issueKeys");

    pub const LABEL: crate::Field<Self, crate::Text> = crate::Field::new("label");

    pub const PIPELINE: crate::Field<Self, crate::Object<super::GetDeploymentByKeyReplyPipeline>> =
        crate::Field::new("pipeline");

    pub const SCHEMA_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("schemaVersion");
    pub const STATE: crate::Field<Self, crate::Text> = crate::Field::new("state");
    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");

    pub const UPDATE_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceNumber");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
