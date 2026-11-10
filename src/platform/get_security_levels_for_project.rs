#[derive(Clone, Copy, Debug)]
pub struct GetSecurityLevelsForProject<'call> {
    pub project_key_or_id: &'call str,
}

impl crate::Operation for GetSecurityLevelsForProject<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectKeyOrId}/securitylevel";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ProjectIssueSecurityLevels>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("projectKeyOrId", self.project_key_or_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
