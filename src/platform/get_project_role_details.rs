#[derive(Clone, Copy, Debug)]
pub struct GetProjectRoleDetails<'call> {
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub exclude_connect_addons: Option<bool>,
    pub exclude_other_service_roles: Option<bool>,
    pub member_current: Option<bool>,
}

impl crate::Operation for GetProjectRoleDetails<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}/roledetails";

    type Options<'options> = Options;
    type Reply = crate::List<crate::Object<super::shape::ProjectRoleDetails>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("excludeConnectAddons", options.exclude_connect_addons);
        request.query_boolean("excludeOtherServiceRoles", options.exclude_other_service_roles);
        request.query_boolean("currentMember", options.member_current);
        request.path_text("projectIdOrKey", self.project_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
