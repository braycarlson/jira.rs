#[derive(Clone, Copy, Debug)]
pub struct RemoveUserFromGroup<'call> {
    pub account_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub group_id: Option<&'options str>,
    pub groupname: Option<&'options str>,
    pub username: Option<&'options str>,
}

impl crate::Operation for RemoveUserFromGroup<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/group/user";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("accountId", Some(self.account_id));
        request.query_text("groupId", options.group_id);
        request.query_text("groupname", options.groupname);
        request.query_text("username", options.username);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
