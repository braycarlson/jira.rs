#[derive(Clone, Copy, Debug)]
pub struct BulkGetUsers<'call> {
    pub account_id: &'call [&'call str],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub key: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub username: Option<&'options [&'options str]>,
}

impl crate::Operation for BulkGetUsers<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/bulk";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanUser>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("accountId", Some(self.account_id));
        request.query_texts("key", options.key);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_texts("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for BulkGetUsers<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::User>;
}
