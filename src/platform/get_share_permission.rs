#[derive(Clone, Copy, Debug)]
pub struct GetSharePermission {
    pub id: i64,
    pub permission_id: i64,
}

impl crate::Operation for GetSharePermission {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/filter/{id}/permission/{permissionId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::SharePermission>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.path_number("permissionId", self.permission_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
