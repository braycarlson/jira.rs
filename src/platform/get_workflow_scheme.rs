#[derive(Clone, Copy, Debug)]
pub struct GetWorkflowScheme {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub return_draft_if_exists: Option<bool>,
}

impl crate::Operation for GetWorkflowScheme {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::WorkflowScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("returnDraftIfExists", options.return_draft_if_exists);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
