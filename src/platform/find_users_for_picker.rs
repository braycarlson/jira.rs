#[derive(Clone, Copy, Debug)]
pub struct FindUsersForPicker<'call> {
    pub query: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub avatar_size: Option<&'options str>,
    pub exclude: Option<&'options [&'options str]>,
    pub exclude_account_ids: Option<&'options [&'options str]>,
    pub exclude_connect_users: Option<bool>,
    pub results_max: Option<i64>,
    pub show_avatar: Option<bool>,
}

impl crate::Operation for FindUsersForPicker<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/picker";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::FoundUsers>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("avatarSize", options.avatar_size);
        request.query_texts("exclude", options.exclude);
        request.query_texts("excludeAccountIds", options.exclude_account_ids);
        request.query_boolean("excludeConnectUsers", options.exclude_connect_users);
        request.query_text("query", Some(self.query));
        request.query_number("maxResults", options.results_max);
        request.query_boolean("showAvatar", options.show_avatar);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
