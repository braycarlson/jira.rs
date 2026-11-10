#[derive(Clone, Copy, Debug)]
pub struct CreateWorkflowSchemeDraftFromParent {
    pub id: i64,
}

impl crate::Operation for CreateWorkflowSchemeDraftFromParent {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/workflowscheme/{id}/createdraft";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::WorkflowScheme>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
