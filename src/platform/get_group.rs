#[derive(Clone, Copy, Debug)]
pub struct GetGroup;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub expand: Option<&'options str>,
    pub group_id: Option<&'options str>,
    pub groupname: Option<&'options str>,
}

impl crate::Operation for GetGroup {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/group";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Group>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("expand", options.expand);
        request.query_text("groupId", options.group_id);
        request.query_text("groupname", options.groupname);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
