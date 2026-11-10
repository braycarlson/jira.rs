#[derive(Clone, Copy, Debug)]
pub struct SearchAndReconsileIssuesUsingJQL;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub fail_fast: Option<bool>,
    pub fields: Option<&'options [&'options str]>,
    pub fields_by_keys: Option<bool>,
    pub include_archived_projects: Option<bool>,
    pub jql: Option<&'options str>,
    pub page_token_next: Option<&'options str>,
    pub properties: Option<&'options [&'options str]>,
    pub reconcile_issues: Option<&'options [i64]>,
    pub results_max: Option<i64>,
}

impl crate::Operation for SearchAndReconsileIssuesUsingJQL {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/search/jql";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchAndReconcileResults>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_boolean("failFast", options.fail_fast);
        request.query_texts("fields", options.fields);
        request.query_boolean("fieldsByKeys", options.fields_by_keys);
        request.query_boolean("includeArchivedProjects", options.include_archived_projects);
        request.query_text("jql", options.jql);
        request.query_text("nextPageToken", options.page_token_next);
        request.query_texts("properties", options.properties);
        request.query_numbers("reconcileIssues", options.reconcile_issues);
        request.query_number("maxResults", options.results_max);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchAndReconsileIssuesUsingJQL {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "nextPageToken",
        items: Some("/issues"),
        next: crate::Next::Token("/nextPageToken"),
    };

    type Item = crate::Object<super::shape::IssueBean>;
}
