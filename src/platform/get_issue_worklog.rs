#[derive(Clone, Copy, Debug)]
pub struct GetIssueWorklog<'call> {
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub started_after: Option<i64>,
    pub started_before: Option<i64>,
}

impl crate::Operation for GetIssueWorklog<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/worklog";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageOfWorklogs>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_number("startedAfter", options.started_after);
        request.query_number("startedBefore", options.started_before);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetIssueWorklog<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/worklogs"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::Worklog>;
}
