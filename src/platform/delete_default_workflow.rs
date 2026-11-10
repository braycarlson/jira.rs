#[derive(Clone, Copy, Debug)]
pub struct DeleteDefaultWorkflow {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_draft_if_needed: Option<bool>,
}

impl crate::Operation for DeleteDefaultWorkflow {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/default";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::WorkflowScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_boolean("updateDraftIfNeeded", options.update_draft_if_needed);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
