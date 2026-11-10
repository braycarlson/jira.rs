#[derive(Clone, Copy, Debug)]
pub struct FindComponentsForProjects;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub order_by: Option<&'options str>,
    pub project_ids_or_keys: Option<&'options [&'options str]>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for FindComponentsForProjects {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/component";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBean2ComponentJSONBean>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("orderBy", options.order_by);
        request.query_texts("projectIdsOrKeys", options.project_ids_or_keys);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for FindComponentsForProjects {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::ComponentJSONBean>;
}
