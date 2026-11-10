#[derive(Clone, Copy, Debug)]
pub struct GetWorkflowSchemeProjectAssociations<'call> {
    pub project_id: &'call [i64],
}

impl crate::Operation for GetWorkflowSchemeProjectAssociations<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflowscheme/project";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContainerOfWorkflowSchemeAssociations>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_numbers("projectId", Some(self.project_id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
