#[derive(Clone, Copy, Debug)]
pub struct GetVisibleIssueFieldOptions<'call> {
    pub field_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub project_id: Option<i64>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetVisibleIssueFieldOptions<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/field/{fieldKey}/option/suggestions/search";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::PageBeanIssueFieldOption>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldKey", self.field_key)?;
        request.query_number("projectId", options.project_id);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetVisibleIssueFieldOptions<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::IssueFieldOption>;
}
