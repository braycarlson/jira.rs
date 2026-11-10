#[derive(Clone, Copy, Debug)]
pub struct GetSharePermissions {
    pub id: i64,
}

impl crate::Operation for GetSharePermissions {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/filter/{id}/permission";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::SharePermission>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
