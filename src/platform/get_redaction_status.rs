#[derive(Clone, Copy, Debug)]
pub struct GetRedactionStatus<'call> {
    pub job_id: &'call str,
}

impl crate::Operation for GetRedactionStatus<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/redact/status/{jobId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::RedactionJobStatusResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("jobId", self.job_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
