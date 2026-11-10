#[derive(Clone, Copy, Debug)]
pub struct DeletePermissionSchemeEntity {
    pub permission_id: i64,
    pub scheme_id: i64,
}

impl crate::Operation for DeletePermissionSchemeEntity {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/permissionscheme/{schemeId}/permission/{permissionId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("permissionId", self.permission_id)?;
        request.path_number("schemeId", self.scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
