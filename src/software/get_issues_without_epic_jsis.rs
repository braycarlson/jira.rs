#[derive(Clone, Copy, Debug)]
pub struct GetIssuesWithoutEpicJSIS;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub fields: Option<&'options [&'options str]>,
    pub jql: Option<&'options str>,
    pub page_token_next: Option<&'options str>,
    pub reconcile_issues: Option<&'options [i64]>,
    pub results_max: Option<i64>,
    pub validate_query: Option<bool>,
}

impl crate::Operation for GetIssuesWithoutEpicJSIS {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/software/1.0/epic/none/issue";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SoftwareIssueResults>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("fields", options.fields);
        request.query_text("jql", options.jql);
        request.query_text("nextPageToken", options.page_token_next);
        request.query_numbers("reconcileIssues", options.reconcile_issues);
        request.query_number("maxResults", options.results_max);
        request.query_boolean("validateQuery", options.validate_query);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetIssuesWithoutEpicJSIS {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "nextPageToken",
        items: Some("/issues"),
        next: crate::Next::Token("/nextPageToken"),
    };

    type Item = crate::Object<super::shape::IssueBean>;
}
