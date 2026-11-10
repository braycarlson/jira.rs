#[derive(Clone, Copy, Debug)]
pub struct DeleteFieldAssociationScheme {
    pub id: i64,
}

impl crate::Operation for DeleteFieldAssociationScheme {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/config/fieldschemes/{id}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::DeleteFieldAssociationSchemeResponse>;

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
