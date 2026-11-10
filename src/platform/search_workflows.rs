#[derive(Clone, Copy, Debug)]
pub struct SearchWorkflows;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub is_active: Option<bool>,
    pub order_by: Option<&'options str>,
    pub project_id: Option<i64>,
    pub query_string: Option<&'options str>,
    pub results_max: Option<i64>,
    pub scope: Option<&'options str>,
    pub start_at: Option<i64>,
}

impl crate::Operation for SearchWorkflows {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflows/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::WorkflowSearchResponse>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_boolean("isActive", options.is_active);
        request.query_text("orderBy", options.order_by);
        request.query_number("projectId", options.project_id);
        request.query_text("queryString", options.query_string);
        request.query_number("maxResults", options.results_max);
        request.query_text("scope", options.scope);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchWorkflows {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::JiraWorkflow>;
}
