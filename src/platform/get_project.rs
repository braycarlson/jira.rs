#[derive(Clone, Copy, Debug)]
pub struct GetProject<'call> {
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub properties: Option<&'options [&'options str]>,
}

impl crate::Operation for GetProject<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Project>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.path_text("projectIdOrKey", self.project_id_or_key)?;
        request.query_texts("properties", options.properties);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
