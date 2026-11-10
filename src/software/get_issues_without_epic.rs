#[derive(Clone, Copy, Debug)]
pub struct GetIssuesWithoutEpic;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub fields: Option<&'options [&'options str]>,
    pub jql: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub validate_query: Option<bool>,
}

impl crate::Operation for GetIssuesWithoutEpic {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/epic/none/issue";

    type Options<'options> = Options<'options>;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("fields", options.fields);
        request.query_text("jql", options.jql);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_boolean("validateQuery", options.validate_query);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetIssuesWithoutEpic {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/issues"), next: crate::Next::Offset };

    type Item = crate::Any;
}
