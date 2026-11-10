#[derive(Clone, Copy, Debug)]
pub struct GetApplicationProperty;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub key: Option<&'options str>,
    pub key_filter: Option<&'options str>,
    pub permission_level: Option<&'options str>,
}

impl crate::Operation for GetApplicationProperty {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/application-properties";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::ApplicationProperty>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("key", options.key);
        request.query_text("keyFilter", options.key_filter);
        request.query_text("permissionLevel", options.permission_level);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
