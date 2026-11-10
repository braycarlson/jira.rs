#[derive(Clone, Copy, Debug)]
pub struct GetCustomerRequestByIDOrKey<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options [&'options str]>,
}

impl crate::Operation for GetCustomerRequestByIDOrKey<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/request/{issueIdOrKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::CustomerRequestDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("expand", options.expand);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
