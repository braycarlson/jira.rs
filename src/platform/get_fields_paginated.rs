#[derive(Clone, Copy, Debug)]
pub struct GetFieldsPaginated;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub id: Option<&'options [&'options str]>,
    pub kind: Option<&'options [&'options str]>,
    pub order_by: Option<&'options str>,
    pub project_ids: Option<&'options [i64]>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetFieldsPaginated {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/field/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanField>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("id", options.id);
        request.query_texts("type", options.kind);
        request.query_text("orderBy", options.order_by);
        request.query_numbers("projectIds", options.project_ids);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetFieldsPaginated {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Field>;
}
