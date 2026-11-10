#[derive(Clone, Copy, Debug)]
pub struct GetIssueSecurityLevelMembers {
    pub issue_security_scheme_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub issue_security_level_id: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetIssueSecurityLevelMembers {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issuesecurityschemes/{issueSecuritySchemeId}/members";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanIssueSecurityLevelMember>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("issueSecurityLevelId", options.issue_security_level_id);
        request.path_number("issueSecuritySchemeId", self.issue_security_scheme_id)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetIssueSecurityLevelMembers {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::IssueSecurityLevelMember>;
}
