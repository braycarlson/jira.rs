#[derive(Clone, Copy, Debug)]
pub struct RemoveCustomers<'call> {
    pub body: &'call serde_json::Value,
    pub service_desk_id: &'call str,
}

impl crate::Operation for RemoveCustomers<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/servicedeskapi/servicedesk/{serviceDeskId}/customer";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("serviceDeskId", self.service_desk_id)?;
        request.header("X-ExperimentalApi", "opt-in");
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
