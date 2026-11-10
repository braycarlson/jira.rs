#[derive(Clone, Copy, Debug)]
pub struct SetWorkflowSchemeIssueType<'call> {
    pub body: &'call serde_json::Value,
    pub id: i64,
    pub issue_type: &'call str,
}

impl crate::Operation for SetWorkflowSchemeIssueType<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/issuetype/{issueType}";

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
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
