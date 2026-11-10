#[derive(Clone, Copy, Debug)]
pub struct GetTheProgressOfAnAsynchronousSchemaAndMappingOperation<'call> {
    pub import_source_id: &'call str,
    pub resource_id: &'call str,
    pub workspace_id: &'call str,
}

impl crate::Operation for GetTheProgressOfAnAsynchronousSchemaAndMappingOperation<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str = concat!(
        "/jsm/assets/workspace/{workspaceId}/v1/importsource/{importSourceId}/mapping/progress/",
        "{resourceId}",
    );

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("importSourceId", self.import_source_id)?;
        request.path_text("resourceId", self.resource_id)?;
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
