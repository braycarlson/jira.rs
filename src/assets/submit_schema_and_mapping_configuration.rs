#[derive(Clone, Copy, Debug)]
pub struct SubmitSchemaAndMappingConfiguration<'call> {
    pub body: &'call serde_json::Value,
    pub import_source_id: &'call str,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub async_value: Option<bool>,
}

impl crate::Operation for SubmitSchemaAndMappingConfiguration<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str =
        "/jsm/assets/workspace/{workspaceId}/v1/importsource/{importSourceId}/mapping";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("async", options.async_value);
        request.path_text("importSourceId", self.import_source_id)?;
        request.path_text("workspaceId", self.workspace_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
