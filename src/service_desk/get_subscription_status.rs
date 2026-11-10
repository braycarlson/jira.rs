#[derive(Clone, Copy, Debug)]
pub struct GetSubscriptionStatus<'call> {
    pub issue_id_or_key: &'call str,
}

impl crate::Operation for GetSubscriptionStatus<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/request/{issueIdOrKey}/notification";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::RequestNotificationSubscriptionDTO>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
