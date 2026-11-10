#[derive(Clone, Copy, Debug)]
pub struct UpdateSchemaAndMappingConfiguration<'call> {
    pub import_source_id: &'call str,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub async_value: Option<bool>,
    pub body: Option<&'options serde_json::Value>,
}

impl crate::Operation for UpdateSchemaAndMappingConfiguration<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Patch;

    const PATH: &'static str =
        "/jsm/assets/workspace/{workspaceId}/v1/importsource/{importSourceId}/mapping";

    type Options<'options> = Options<'options>;
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

        if let Some(body) = options.body {
            request.body_json(body)?;
        }

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
