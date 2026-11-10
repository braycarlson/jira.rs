#[derive(Clone, Copy, Debug)]
pub struct GetPlans;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub cursor: Option<&'options str>,
    pub include_archived: Option<bool>,
    pub include_trashed: Option<bool>,
    pub results_max: Option<i64>,
}

impl crate::Operation for GetPlans {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/plans/plan";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageWithCursorGetPlanResponseForPage>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("cursor", options.cursor);
        request.query_boolean("includeArchived", options.include_archived);
        request.query_boolean("includeTrashed", options.include_trashed);
        request.query_number("maxResults", options.results_max);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetPlans {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: Some("/values"),
        next: crate::Next::Token("/nextPageCursor"),
    };

    type Item = crate::Object<super::shape::GetPlanResponseForPage>;
}
