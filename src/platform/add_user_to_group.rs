#[derive(Clone, Copy, Debug)]
pub struct AddUserToGroup<'call> {
    pub body: &'call serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub group_id: Option<&'options str>,
    pub groupname: Option<&'options str>,
}

impl crate::Operation for AddUserToGroup<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/group/user";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Group>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("groupId", options.group_id);
        request.query_text("groupname", options.groupname);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
