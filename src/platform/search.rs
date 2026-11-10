#[derive(Clone, Copy, Debug)]
pub struct Search;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub include_global_statuses: Option<bool>,
    pub project_id: Option<&'options str>,
    pub results_max: Option<i64>,
    pub search_string: Option<&'options str>,
    pub start_at: Option<i64>,
    pub status_category: Option<&'options str>,
}

impl crate::Operation for Search {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/statuses/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageOfStatuses>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("includeGlobalStatuses", options.include_global_statuses);
        request.query_text("projectId", options.project_id);
        request.query_number("maxResults", options.results_max);
        request.query_text("searchString", options.search_string);
        request.query_number("startAt", options.start_at);
        request.query_text("statusCategory", options.status_category);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for Search {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::JiraStatus>;
}
