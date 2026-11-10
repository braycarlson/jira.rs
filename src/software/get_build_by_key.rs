#[derive(Clone, Copy, Debug)]
pub struct GetBuildByKey<'call> {
    pub build_number: i64,
    pub pipeline_id: &'call str,
}

impl crate::Operation for GetBuildByKey<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/builds/0.1/pipelines/{pipelineId}/builds/{buildNumber}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetBuildByKeyReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("buildNumber", self.build_number)?;
        request.path_text("pipelineId", self.pipeline_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
