#[derive(Clone, Copy, Debug)]
pub struct DeleteDeploymentByKey<'call> {
    pub deployment_sequence_number: i64,
    pub environment_id: &'call str,
    pub pipeline_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_sequence_number: Option<i64>,
}

impl crate::Operation for DeleteDeploymentByKey<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str = concat!(
        "/rest/deployments/0.1/pipelines/{pipelineId}/environments/{environmentId}/deployments/",
        "{deploymentSequenceNumber}",
    );

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("deploymentSequenceNumber", self.deployment_sequence_number)?;
        request.path_text("environmentId", self.environment_id)?;
        request.path_text("pipelineId", self.pipeline_id)?;
        request.query_number("_updateSequenceNumber", options.update_sequence_number);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
