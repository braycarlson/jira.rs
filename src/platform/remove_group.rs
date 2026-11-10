#[derive(Clone, Copy, Debug)]
pub struct RemoveGroup;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub group_id: Option<&'options str>,
    pub groupname: Option<&'options str>,
    pub swap_group: Option<&'options str>,
    pub swap_group_id: Option<&'options str>,
}

impl crate::Operation for RemoveGroup {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/group";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("groupId", options.group_id);
        request.query_text("groupname", options.groupname);
        request.query_text("swapGroup", options.swap_group);
        request.query_text("swapGroupId", options.swap_group_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
