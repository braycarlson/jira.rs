#[derive(Clone, Copy, Debug)]
pub struct GetWorkflow {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub return_draft_if_exists: Option<bool>,
    pub workflow_name: Option<&'options str>,
}

impl crate::Operation for GetWorkflow {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/workflow";

    type Options<'options> = Options<'options>;
    type Reply = crate::Any;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("returnDraftIfExists", options.return_draft_if_exists);
        request.query_text("workflowName", options.workflow_name);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
