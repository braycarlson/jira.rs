#[derive(Clone, Copy, Debug)]
pub struct CreateCustomerSkipPermissionCheck<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub strict_conflict_status_code: Option<bool>,
}

impl crate::Operation for CreateCustomerSkipPermissionCheck<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/servicedeskapi/customer/skip-permission-check";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::UserDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("strictConflictStatusCode", options.strict_conflict_status_code);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
