#[derive(Clone, Copy, Debug)]
pub struct FindUsers;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub property: Option<&'options str>,
    pub query: Option<&'options str>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
    pub username: Option<&'options str>,
}

impl crate::Operation for FindUsers {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/search";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::User>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.query_text("property", options.property);
        request.query_text("query", options.query);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
