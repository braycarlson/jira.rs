#[derive(Clone, Copy, Debug)]
pub struct FindGroups;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub case_insensitive: Option<bool>,
    pub exclude: Option<&'options [&'options str]>,
    pub exclude_id: Option<&'options [&'options str]>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub user_name: Option<&'options str>,
}

impl crate::Operation for FindGroups {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/groups/picker";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::FoundGroups>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.query_boolean("caseInsensitive", options.case_insensitive);
        request.query_texts("exclude", options.exclude);
        request.query_texts("excludeId", options.exclude_id);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_text("userName", options.user_name);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
