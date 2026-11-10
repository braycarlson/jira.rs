#[derive(Clone, Copy, Debug)]
pub struct DeleteFeedback<'call> {
    pub request_id_or_key: &'call str,
}

impl crate::Operation for DeleteFeedback<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/servicedeskapi/request/{requestIdOrKey}/feedback";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("requestIdOrKey", self.request_id_or_key)?;
        request.header("X-ExperimentalApi", "opt-in");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
