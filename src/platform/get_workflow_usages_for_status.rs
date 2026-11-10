#[derive(Clone, Copy, Debug)]
pub struct GetWorkflowUsagesForStatus<'call> {
    pub status_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub page_token_next: Option<&'options str>,
    pub results_max: Option<i64>,
}

impl crate::Operation for GetWorkflowUsagesForStatus<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/statuses/{statusId}/workflowUsages";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::StatusWorkflowUsageDTO>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("nextPageToken", options.page_token_next);
        request.query_number("maxResults", options.results_max);
        request.path_text("statusId", self.status_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetWorkflowUsagesForStatus<'_> {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "nextPageToken",
        items: Some("/workflows/values"),
        next: crate::Next::Token("/workflows/nextPageToken"),
    };

    type Item = crate::Object<super::shape::StatusWorkflowUsageWorkflow>;
}
