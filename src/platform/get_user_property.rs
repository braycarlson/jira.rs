#[derive(Clone, Copy, Debug)]
pub struct GetUserProperty<'call> {
    pub property_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub account_id: Option<&'options str>,
    pub user_key: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for GetUserProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/user/properties/{propertyKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::EntityProperty>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", options.account_id);
        request.path_text("propertyKey", self.property_key)?;
        request.query_text("userKey", options.user_key);
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
