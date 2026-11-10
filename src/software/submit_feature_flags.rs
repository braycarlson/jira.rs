#[derive(Clone, Copy, Debug)]
pub struct SubmitFeatureFlags<'call> {
    pub body: &'call serde_json::Value,
}

impl crate::Operation for SubmitFeatureFlags<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/featureflags/0.1/bulk";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::SubmitFeatureFlagsReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
