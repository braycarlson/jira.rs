#[derive(Clone, Copy, Debug)]
pub struct GetCreateIssueMetaIssueTypes<'call> {
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetCreateIssueMetaIssueTypes<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/createmeta/{projectIdOrKey}/issuetypes";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PageOfCreateMetaIssueTypes>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("projectIdOrKey", self.project_id_or_key)?;
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetCreateIssueMetaIssueTypes<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/issueTypes"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::IssueTypeIssueCreateMetadata>;
}
