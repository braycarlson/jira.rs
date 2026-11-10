#[derive(Clone, Copy, Debug)]
pub struct ObjectUpdate<'call> {
    pub body: &'call serde_json::Value,
    pub id: &'call str,
    pub workspace_id: &'call str,
}

impl crate::Operation for ObjectUpdate<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/object/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::Object>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("id", self.id)?;
        request.path_text("workspaceId", self.workspace_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
