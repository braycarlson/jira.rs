#[derive(Clone, Copy, Debug)]
pub struct GetProperty<'call> {
    pub organization_id: &'call str,
    pub property_key: &'call str,
}

impl crate::Operation for GetProperty<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;

    const PATH: &'static str =
        "/rest/servicedeskapi/organization/{organizationId}/property/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::EntityProperty>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("organizationId", self.organization_id)?;
        request.path_text("propertyKey", self.property_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
