#[derive(Clone, Copy, Debug)]
pub struct GetUsersFromGroup;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub group_id: Option<&'options str>,
    pub groupname: Option<&'options str>,
    pub include_inactive_users: Option<bool>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetUsersFromGroup {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/group/member";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBeanUserDetails>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("groupId", options.group_id);
        request.query_text("groupname", options.groupname);
        request.query_boolean("includeInactiveUsers", options.include_inactive_users);
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetUsersFromGroup {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::UserDetails>;
}
