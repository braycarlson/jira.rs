#[derive(Clone, Copy, Debug)]
pub struct GetPermissionScheme {
    pub scheme_id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
}

impl crate::Operation for GetPermissionScheme {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/permissionscheme/{schemeId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PermissionScheme>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.path_number("schemeId", self.scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
