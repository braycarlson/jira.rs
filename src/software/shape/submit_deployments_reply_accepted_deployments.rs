#[derive(Clone, Copy, Debug)]
pub struct SubmitDeploymentsReplyAcceptedDeployments;

impl SubmitDeploymentsReplyAcceptedDeployments {
    pub const DEPLOYMENT_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("deploymentSequenceNumber");

    pub const ENVIRONMENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("environmentId");
    pub const PIPELINE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pipelineId");
}
