#[derive(Clone, Copy, Debug)]
pub struct GetFieldAssociationSchemeItemParameters<'call> {
    pub field_id: &'call str,
    pub id: i64,
}

impl crate::Operation for GetFieldAssociationSchemeItemParameters<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/config/fieldschemes/{id}/fields/{fieldId}/parameters";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetFieldAssociationParametersResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("fieldId", self.field_id)?;
        request.path_number("id", self.id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
