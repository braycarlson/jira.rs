#[derive(Clone, Copy, Debug)]
pub struct DeleteProjectRoleActorsFromRole {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub group: Option<&'options str>,
    pub group_id: Option<&'options str>,
    pub user: Option<&'options str>,
}

impl crate::Operation for DeleteProjectRoleActorsFromRole {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/role/{id}/actors";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::ProjectRole>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("group", options.group);
        request.query_text("groupId", options.group_id);
        request.path_number("id", self.id)?;
        request.query_text("user", options.user);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
