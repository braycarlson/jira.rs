#[derive(Clone, Copy, Debug)]
pub struct GetOrganization {
    pub organization_id: i64,
}

impl crate::Operation for GetOrganization {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/organization/{organizationId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::OrganizationDTO>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("organizationId", self.organization_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
