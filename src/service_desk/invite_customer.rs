#[derive(Clone, Copy, Debug)]
pub struct InviteCustomer<'call> {
    pub body: &'call serde_json::Value,
    pub service_desk_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub strict_conflict_status_code: Option<bool>,
}

impl crate::Operation for InviteCustomer<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/servicedeskapi/servicedesk/{serviceDeskId}/customer/invite";

    type Options<'options> = Options;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.query_boolean("strictConflictStatusCode", options.strict_conflict_status_code);
        request.header("X-ExperimentalApi", "opt-in");
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
