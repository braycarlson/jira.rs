#[derive(Clone, Copy, Debug)]
pub struct UpdateWorkflowMapping<'call> {
    pub body: &'call serde_json::Value,
    pub id: i64,
    pub workflow_name: &'call str,
}

impl crate::Operation for UpdateWorkflowMapping<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/workflow";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::WorkflowScheme>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_text("workflowName", Some(self.workflow_name));
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
