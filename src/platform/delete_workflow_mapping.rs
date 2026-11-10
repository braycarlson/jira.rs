#[derive(Clone, Copy, Debug)]
pub struct DeleteWorkflowMapping<'call> {
    pub id: i64,
    pub workflow_name: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_draft_if_needed: Option<bool>,
}

impl crate::Operation for DeleteWorkflowMapping<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/workflow";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("updateDraftIfNeeded", options.update_draft_if_needed);
        request.query_text("workflowName", Some(self.workflow_name));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
