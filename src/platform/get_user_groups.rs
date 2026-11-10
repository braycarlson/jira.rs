#[derive(Clone, Copy, Debug)]
pub struct GetUserGroups<'call> {
    pub account_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub key: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for GetUserGroups<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/groups";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::GroupName>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", Some(self.account_id));
        request.query_text("key", options.key);
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
