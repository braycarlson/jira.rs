#[derive(Clone, Copy, Debug)]
pub struct GetWorkflowSchemeUsagesForWorkflow<'call> {
    pub workflow_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub page_token_next: Option<&'options str>,
    pub results_max: Option<i64>,
}

impl crate::Operation for GetWorkflowSchemeUsagesForWorkflow<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflow/{workflowId}/workflowSchemes";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::WorkflowSchemeUsageDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("nextPageToken", options.page_token_next);
        request.query_number("maxResults", options.results_max);
        request.path_text("workflowId", self.workflow_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetWorkflowSchemeUsagesForWorkflow<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "nextPageToken",
        items: Some("/workflowSchemes/values"),
        next: crate::Next::Token("/workflowSchemes/nextPageToken"),
    };

    type Item = crate::Object<super::shape::WorkflowSchemeUsage>;
}
