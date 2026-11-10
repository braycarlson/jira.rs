#[derive(Clone, Copy, Debug)]
pub struct GetProjectRole<'call> {
    pub id: i64,
    pub project_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub exclude_inactive_users: Option<bool>,
}

impl crate::Operation for GetProjectRole<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}/role/{id}";

    type Options<'options> = Options;
    type Reply = crate::Object<super::shape::ProjectRole>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("excludeInactiveUsers", options.exclude_inactive_users);
        request.path_number("id", self.id)?;
        request.path_text("projectIdOrKey", self.project_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
