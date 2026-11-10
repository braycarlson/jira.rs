#[derive(Clone, Copy, Debug)]
pub struct MigrationResourceWorkflowRuleSearchPost<'call> {
    pub atlassian_transfer_id: &'call str,
    pub body: &'call serde_json::Value,
}

impl crate::Operation for MigrationResourceWorkflowRuleSearchPost<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/atlassian-connect/1/migration/workflow/rule/search";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::WorkflowRulesSearchDetails>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.header("Atlassian-Transfer-Id", self.atlassian_transfer_id);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
