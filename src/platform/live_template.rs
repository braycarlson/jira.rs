#[derive(Clone, Copy, Debug)]
pub struct LiveTemplate;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub project_id: Option<&'options str>,
    pub template_key: Option<&'options str>,
}

impl crate::Operation for LiveTemplate {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project-template/live-template";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ProjectTemplateModel>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("projectId", options.project_id);
        request.query_text("templateKey", options.template_key);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
