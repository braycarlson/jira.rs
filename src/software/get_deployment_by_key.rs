#[derive(Clone, Copy, Debug)]
pub struct GetDeploymentByKey<'call> {
    pub deployment_sequence_number: i64,
    pub environment_id: &'call str,
    pub pipeline_id: &'call str,
}

impl crate::Operation for GetDeploymentByKey<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str = concat!(
        "/rest/deployments/0.1/pipelines/{pipelineId}/environments/{environmentId}/deployments/",
        "{deploymentSequenceNumber}",
    );

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetDeploymentByKeyReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("deploymentSequenceNumber", self.deployment_sequence_number)?;
        request.path_text("environmentId", self.environment_id)?;
        request.path_text("pipelineId", self.pipeline_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
