#[derive(Clone, Copy, Debug)]
pub struct DeleteWorkflowSchemeIssueType<'call> {
    pub id: i64,
    pub issue_type: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub update_draft_if_needed: Option<bool>,
}

impl crate::Operation for DeleteWorkflowSchemeIssueType<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/issuetype/{issueType}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::WorkflowScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.path_text("issueType", self.issue_type)?;
        request.query_boolean("updateDraftIfNeeded", options.update_draft_if_needed);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
