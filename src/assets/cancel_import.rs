#[derive(Clone, Copy, Debug)]
pub struct CancelImport<'call> {
    pub import_execution_id: &'call str,
    pub import_source_id: &'call str,
    pub workspace_id: &'call str,
}

impl crate::Operation for CancelImport<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str = concat!(
        "/jsm/assets/workspace/{workspaceId}/v1/importsource/{importSourceId}/executions/",
        "{importExecutionId}",
    );

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("importExecutionId", self.import_execution_id)?;
        request.path_text("importSourceId", self.import_source_id)?;
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
