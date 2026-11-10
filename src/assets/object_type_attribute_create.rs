#[derive(Clone, Copy, Debug)]
pub struct ObjectTypeAttributeCreate<'call> {
    pub body: &'call serde_json::Value,
    pub object_type_id: &'call str,
    pub workspace_id: &'call str,
}

impl crate::Operation for ObjectTypeAttributeCreate<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;

    const PATH: &'static str =
        "/jsm/assets/workspace/{workspaceId}/v1/objecttypeattribute/{objectTypeId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ObjectTypeAttribute>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("objectTypeId", self.object_type_id)?;
        request.path_text("workspaceId", self.workspace_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
