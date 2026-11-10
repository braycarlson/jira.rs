#[derive(Clone, Copy, Debug)]
pub struct GetWorkflowsPaginated;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub is_active: Option<bool>,
    pub order_by: Option<&'options str>,
    pub query_string: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub workflow_name: Option<&'options [&'options str]>,
}

impl crate::Operation for GetWorkflowsPaginated {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflow/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanWorkflow>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_boolean("isActive", options.is_active);
        request.query_text("orderBy", options.order_by);
        request.query_text("queryString", options.query_string);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_texts("workflowName", options.workflow_name);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetWorkflowsPaginated {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Workflow>;
}
