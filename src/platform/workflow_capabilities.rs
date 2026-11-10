#[derive(Clone, Copy, Debug)]
pub struct WorkflowCapabilities;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub issue_type_id: Option<&'options str>,
    pub project_id: Option<&'options str>,
    pub workflow_id: Option<&'options str>,
}

impl crate::Operation for WorkflowCapabilities {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflows/capabilities";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::WorkflowCapabilities>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("issueTypeId", options.issue_type_id);
        request.query_text("projectId", options.project_id);
        request.query_text("workflowId", options.workflow_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
