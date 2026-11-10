#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentGatingStatusByKeyReply;

impl GetDeploymentGatingStatusByKeyReply {
    pub const DEPLOYMENT_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("deploymentSequenceNumber");

    pub const DETAILS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetDeploymentGatingStatusByKeyReplyDetails>>,
    > = crate::Field::new("details");

    pub const ENVIRONMENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("environmentId");
    pub const GATING_STATUS: crate::Field<Self, crate::Text> = crate::Field::new("gatingStatus");
    pub const PIPELINE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pipelineId");

    pub const UPDATED_TIMESTAMP: crate::Field<Self, crate::Text> =
        crate::Field::new("updatedTimestamp");
}
