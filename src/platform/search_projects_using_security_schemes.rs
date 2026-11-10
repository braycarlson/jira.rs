#[derive(Clone, Copy, Debug)]
pub struct SearchProjectsUsingSecuritySchemes;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub issue_security_scheme_id: Option<&'options [&'options str]>,
    pub project_id: Option<&'options [&'options str]>,
    pub results_max: Option<&'options str>,
    pub start_at: Option<&'options str>,
}

impl crate::Operation for SearchProjectsUsingSecuritySchemes {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issuesecurityschemes/project";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanIssueSecuritySchemeToProjectMapping>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("issueSecuritySchemeId", options.issue_security_scheme_id);
        request.query_texts("projectId", options.project_id);
        request.query_text("maxResults", options.results_max);
        request.query_text("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchProjectsUsingSecuritySchemes {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::IssueSecuritySchemeToProjectMapping>;
}
