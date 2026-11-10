#[derive(Clone, Copy, Debug)]
pub struct PublishDraftWorkflowScheme<'call> {
    pub body: &'call serde_json::Value,
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub validate_only: Option<bool>,
}

impl crate::Operation for PublishDraftWorkflowScheme<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/draft/publish";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::TaskProgressBeanObject>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("validateOnly", options.validate_only);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
