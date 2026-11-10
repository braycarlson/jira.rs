#[derive(Clone, Copy, Debug)]
pub struct SchemaCreate<'call> {
    pub body: &'call serde_json::Value,
    pub workspace_id: &'call str,
}

impl crate::Operation for SchemaCreate<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/objectschema/create";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ObjectSchema>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("workspaceId", self.workspace_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
