#[derive(Clone, Copy, Debug)]
pub struct GetPropertiesKeys<'call> {
    pub organization_id: &'call str,
}

impl crate::Operation for GetPropertiesKeys<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/organization/{organizationId}/property";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PropertyKeys>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("organizationId", self.organization_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
