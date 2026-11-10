#[derive(Clone, Copy, Debug)]
pub struct ListPolicies;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for ListPolicies {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/policies";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PolicySummaryResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for ListPolicies {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: Some("/values"),
        next: crate::Next::Token("/nextPageCursor"),
    };

    type Item = crate::Object<super::shape::PolicySummary>;
}
