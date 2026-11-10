#[derive(Clone, Copy, Debug)]
pub struct SearchForIssuesUsingJQL;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub fail_fast: Option<bool>,
    pub fields: Option<&'options [&'options str]>,
    pub fields_by_keys: Option<bool>,
    pub jql: Option<&'options str>,
    pub properties: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub validate_query: Option<&'options str>,
}

impl crate::Operation for SearchForIssuesUsingJQL {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::SearchResults>;

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
        request.query_text("jql", options.jql);
        request.query_texts("properties", options.properties);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_text("validateQuery", options.validate_query);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for SearchForIssuesUsingJQL {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/issues"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::IssueBean>;
}
