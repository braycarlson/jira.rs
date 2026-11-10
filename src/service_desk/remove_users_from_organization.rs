#[derive(Clone, Copy, Debug)]
pub struct RemoveUsersFromOrganization<'call> {
    pub body: &'call serde_json::Value,
    pub organization_id: i64,
}

impl crate::Operation for RemoveUsersFromOrganization<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/servicedeskapi/organization/{organizationId}/user";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("organizationId", self.organization_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
