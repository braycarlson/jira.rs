#[derive(Clone, Copy, Debug)]
pub struct GetProjectUsagesForWorkflowScheme<'call> {
    pub workflow_scheme_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub page_token_next: Option<&'options str>,
    pub results_max: Option<i64>,
}

impl crate::Operation for GetProjectUsagesForWorkflowScheme<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{workflowSchemeId}/projectUsages";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::WorkflowSchemeProjectUsageDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("nextPageToken", options.page_token_next);
        request.query_number("maxResults", options.results_max);
        request.path_text("workflowSchemeId", self.workflow_scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetProjectUsagesForWorkflowScheme<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "nextPageToken",
        items: Some("/projects/values"),
        next: crate::Next::Token("/projects/nextPageToken"),
    };

    type Item = crate::Object<super::shape::ProjectUsage>;
}
