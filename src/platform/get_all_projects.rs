#[derive(Clone, Copy, Debug)]
pub struct GetAllProjects;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub properties: Option<&'options [&'options str]>,
    pub recent: Option<i64>,
}

impl crate::Operation for GetAllProjects {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project";

    type Options<'options> = Options<'options>;
    type Reply = crate::List<crate::Object<super::shape::Project>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("properties", options.properties);
        request.query_number("recent", options.recent);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
