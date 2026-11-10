#[derive(Clone, Copy, Debug)]
pub struct ObjectFindHistoryEntries<'call> {
    pub id: &'call str,
    pub workspace_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub asc: Option<bool>,
}

impl crate::Operation for ObjectFindHistoryEntries<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/jsm/assets/workspace/{workspaceId}/v1/object/{id}/history";

    type Options<'options> = Options;
    type Reply = crate::List<crate::Object<super::shape::ObjectHistory>>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_boolean("asc", options.asc);
        request.path_text("id", self.id)?;
        request.path_text("workspaceId", self.workspace_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
