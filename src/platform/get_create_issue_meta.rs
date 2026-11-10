#[derive(Clone, Copy, Debug)]
pub struct GetCreateIssueMeta;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub issuetype_ids: Option<&'options [&'options str]>,
    pub issuetype_names: Option<&'options [&'options str]>,
    pub project_ids: Option<&'options [&'options str]>,
    pub project_keys: Option<&'options [&'options str]>,
}

impl crate::Operation for GetCreateIssueMeta {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issue/createmeta";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::IssueCreateMetadata>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_texts("issuetypeIds", options.issuetype_ids);
        request.query_texts("issuetypeNames", options.issuetype_names);
        request.query_texts("projectIds", options.project_ids);
        request.query_texts("projectKeys", options.project_keys);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
