#[derive(Clone, Copy, Debug)]
pub struct ValidateProjectKey;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub key: Option<&'options str>,
}

impl crate::Operation for ValidateProjectKey {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/projectvalidate/key";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ErrorCollection>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("key", options.key);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
