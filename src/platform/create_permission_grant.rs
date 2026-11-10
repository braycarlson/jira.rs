#[derive(Clone, Copy, Debug)]
pub struct CreatePermissionGrant<'call> {
    pub body: &'call serde_json::Value,
    pub scheme_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
}

impl crate::Operation for CreatePermissionGrant<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/permissionscheme/{schemeId}/permission";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PermissionGrant>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.path_number("schemeId", self.scheme_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
