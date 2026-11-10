#[derive(Clone, Copy, Debug)]
pub struct GetAuditEntries;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub page_token_next: Option<&'options str>,
    pub results_max: Option<i64>,
    pub user_account_id: Option<&'options str>,
}

impl crate::Operation for GetAuditEntries {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/invalid-jqls-access-log";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::AuditPage>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("nextPageToken", options.page_token_next);
        request.query_number("maxResults", options.results_max);
        request.query_text("userAccountId", options.user_account_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetAuditEntries {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "nextPageToken",
        items: Some("/entries"),
        next: crate::Next::Token("/nextPageToken"),
    };

    type Item = crate::Object<super::shape::AuditEntry>;
}
