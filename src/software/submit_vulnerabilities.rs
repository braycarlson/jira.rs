#[derive(Clone, Copy, Debug)]
pub struct SubmitVulnerabilities<'call> {
    pub body: &'call serde_json::Value,
}

impl crate::Operation for SubmitVulnerabilities<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/security/1.0/bulk";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::SubmitVulnerabilitiesReply>;

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
