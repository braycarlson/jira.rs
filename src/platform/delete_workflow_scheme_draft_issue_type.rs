#[derive(Clone, Copy, Debug)]
pub struct DeleteWorkflowSchemeDraftIssueType<'call> {
    pub id: i64,
    pub issue_type: &'call str,
}

impl crate::Operation for DeleteWorkflowSchemeDraftIssueType<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/draft/issuetype/{issueType}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::WorkflowScheme>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.path_text("issueType", self.issue_type)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
